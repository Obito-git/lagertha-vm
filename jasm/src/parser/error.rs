use crate::error::{JasmDiagnostic, JasmError};
use crate::token::{JasmToken, JasmTokenKind, Span};
use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum ParserError {
    ClassDirectiveExpected(Span, JasmTokenKind),
    TrailingTokens(Vec<JasmToken>, TrailingTokensContext),
    IdentifierExpected(Span, JasmTokenKind, IdentifierContext),

    MethodDescriptorExpected(Span, JasmTokenKind, MethodDescriptorContext),

    UnexpectedCodeDirectiveArg(Span, JasmTokenKind),

    NonNegativeIntegerExpected(Span, JasmTokenKind, NonNegativeIntegerContext),

    UnknownInstruction(Span, String),

    EmptyFile(Span),
    Internal(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum NonNegativeIntegerContext {
    CodeLocals,
    CodeStack,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum TrailingTokensContext {
    Class,
    Super,
    Method,
    Code,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum IdentifierContext {
    ClassDirective,
    SuperDirective,
    MethodDirectiveName,
    InstructionName,
    ClassNameInstructionArg,
    MethodNameInstructionArg,
    FieldNameInstructionArg,
    FieldDescriptorInstructionArg,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum MethodDescriptorContext {
    MethodDirective,
    Instruction,
}

impl ParserError {
    pub fn message(&self) -> Option<String> {
        match self {
            ParserError::ClassDirectiveExpected(_, token) => Some(format!(
                "Unexpected {} before class definition",
                token.as_string_token_type()
            )),
            ParserError::TrailingTokens(_, _) => Some("trailing characters".to_string()),
            ParserError::IdentifierExpected(_, _, context) => Some(
                match context {
                    IdentifierContext::ClassDirective => "incomplete class definition",
                    IdentifierContext::SuperDirective => "incomplete .super directive",
                    IdentifierContext::MethodDirectiveName => "incomplete .method directive",
                    _ => unimplemented!(),
                }
                .to_string(),
            ),
            ParserError::EmptyFile(_) => Some("File contains no class definition".to_string()),
            ParserError::Internal(msg) => Some(format!("Internal parser error: {}", msg)),
            _ => unimplemented!(),
        }
    }

    pub fn label(&self) -> Option<String> {
        match self {
            ParserError::TrailingTokens(_, _) => {
                Some("Class headers must end after the name.".to_string())
            }
            ParserError::ClassDirectiveExpected(_, token) => match token {
                JasmTokenKind::DotMethod | JasmTokenKind::DotSuper => Some(format!(
                    "The '{}' directive is only allowed inside a class definition.",
                    token
                )),
                JasmTokenKind::DotCode => Some(format!(
                    "The '{}' directive is only allowed inside a method definition.",
                    token
                )),
                JasmTokenKind::DotEnd => Some(format!(
                    "The '{}' directive has no matching start directive.",
                    token
                )),
                _ => Some(format!(
                    "The '{}' {} must appear inside a class definition.",
                    token,
                    token.as_string_token_type()
                )),
            },
            ParserError::IdentifierExpected(_, _, context) => {
                Some(
                    match context {
                        IdentifierContext::ClassDirective =>
                            "The .class directive requires a class name after the access flags.",
                        IdentifierContext::SuperDirective =>
                            "The .super directive requires a superclass name.",
                        IdentifierContext::MethodDirectiveName =>
                            "The .method directive requires a method name followed by parentheses and a method descriptor.",
                        _ => unimplemented!()
                    }.to_string())
            }
            ParserError::Internal(_) => None,
            ParserError::EmptyFile(_) => Some("The file is empty or contains only comments.".to_string()),
            _ => unimplemented!()
        }
    }

    pub fn note(&self) -> Option<String> {
        match self {
            ParserError::ClassDirectiveExpected(_, token) => match token {
                JasmTokenKind::DotMethod | JasmTokenKind::DotSuper => {
                    Some("Define a class first using '.class [access_flags] <name>'.".to_string())
                }
                JasmTokenKind::DotCode => Some(
                    "Define a method first using '.method [access_flags] <name> <descriptor>'."
                        .to_string(),
                ),
                JasmTokenKind::DotEnd => Some(
                    "The '.end' directive must match a previous '.method', '.code', or '.class' directive.".to_string(),
                ),
                JasmTokenKind::Public | JasmTokenKind::Static => Some(
                    "Keywords like 'public' and 'static' are access modifiers that must appear within '.class' or '.method' directives.".to_string(),
                ),
                JasmTokenKind::Identifier(_) => Some(
                    "Identifiers (class, method, or field names) must be used within appropriate directives like '.class', '.method', or field references.".to_string(),
                ),
                JasmTokenKind::Integer(_) => Some(
                    "Integer literals are typically used as instruction arguments inside '.code' blocks.".to_string(),
                ),
                JasmTokenKind::StringLiteral(_) => Some(
                    "String literals are constant values that must appear inside '.code' blocks as instruction arguments.".to_string(),
                ),
                JasmTokenKind::MethodDescriptor(_) => Some(
                    "Method descriptors specify method signatures and must appear after method names in '.method' directives or as instruction arguments.".to_string(),
                ),
                _ => {
                    Some("All assembly code must be placed inside a class definition starting with '.class'.".to_string())
                }
            },
            ParserError::TrailingTokens(tokens, context) => Some(format!(
                "The class definition should end after the class name.\n{}",
                match (tokens[0].kind.clone(), context) {
                    (JasmTokenKind::DotSuper, TrailingTokensContext::Class) =>
                        "Consider starting a new line for the '.super' directive.",
                    (JasmTokenKind::DotMethod, TrailingTokensContext::Class) =>
                        "Consider starting a new line for the '.method' directive.",
                    (
                        JasmTokenKind::Public | JasmTokenKind::Static,
                        TrailingTokensContext::Class,
                    ) =>
                        "Access flags must appear before the class name:\n.class [access_flags] <name>",
                    _ =>
                        "Unexpected tokens after class name. Consider starting a new line for the next directive.",
                }
            )),
            ParserError::IdentifierExpected(_, kind, context) => match (kind, context) {
                (
                    JasmTokenKind::StringLiteral(_),
                    IdentifierContext::ClassDirective | IdentifierContext::SuperDirective,
                ) => Some("Consider removing the quotes around the class name".to_string()),
                (JasmTokenKind::StringLiteral(_), IdentifierContext::MethodDirectiveName) => {
                    Some("Consider removing the quotes around the method name".to_string())
                }
                _ => Some(
                    "The .class directive requires a name:\n.class [access_flags] <name>"
                        .to_string(),
                ),
            },
            ParserError::EmptyFile(_) => Some("A Java assembly file must start with a '.class' directive.".to_string()),
            ParserError::Internal(_) => None,
            _ => unimplemented!(),
        }
    }

    pub fn as_range(&self) -> Option<Range<usize>> {
        self.span().map(|s| s.as_range())
    }

    fn span(&self) -> Option<Span> {
        match self {
            ParserError::ClassDirectiveExpected(span, _)
            | ParserError::EmptyFile(span)
            | ParserError::IdentifierExpected(span, _, _) => Some(*span),
            ParserError::TrailingTokens(tokens, _) => Some(Span::new(
                tokens[0].span.start,
                tokens.last().map(|v| v.span.end).unwrap_or(0),
            )),
            ParserError::Internal(_) => None,
            _ => unimplemented!(),
        }
    }
}

impl From<ParserError> for JasmError {
    fn from(err: ParserError) -> Self {
        match err {
            ParserError::Internal(msg) => JasmError::Internal(msg),
            _ => JasmError::Diagnostic(JasmDiagnostic::new(
                err.message().unwrap_or("parsing error".to_string()),
                err.as_range(),
                err.note(),
                err.label(),
            )),
        }
    }
}
