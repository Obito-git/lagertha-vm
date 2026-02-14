use crate::diagnostic::{Diagnostic, DiagnosticLabel, Severity};
use crate::token::Span;
use std::ops::Range;

#[derive(Debug)]
pub(super) enum ParserWarning {
    MissingSuperClass {
        class_name: String,
        class_directive_pos: Span,
        default: &'static str,
    },
}

impl ParserWarning {
    fn get_message(&self) -> String {
        match self {
            ParserWarning::MissingSuperClass { .. } => "missing super directive".to_string(),
        }
    }

    fn get_labels(&self) -> Vec<DiagnosticLabel> {
        match self {
            ParserWarning::MissingSuperClass {
                class_directive_pos,
                class_name,
                ..
            } => vec![DiagnosticLabel::at(
                class_directive_pos.as_range(),
                format!("Class {} is missing a superclass directive", class_name),
            )],
        }
    }

    fn get_note(&self) -> Option<String> {
        match self {
            ParserWarning::MissingSuperClass { default, .. } => Some(format!(
                "The .super directive is required to specify the superclass. Defaulting to {}.",
                default
            )),
        }
    }

    fn get_primary_location(&self) -> Range<usize> {
        match self {
            ParserWarning::MissingSuperClass {
                class_directive_pos,
                ..
            } => class_directive_pos.as_range(),
        }
    }
}

impl Diagnostic for ParserWarning {
    fn message(&self) -> String {
        self.get_message()
    }

    fn primary_location(&self) -> Range<usize> {
        self.get_primary_location()
    }

    fn note(&self) -> Option<String> {
        self.get_note()
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn labels(&self) -> Vec<DiagnosticLabel> {
        self.get_labels()
    }
}
