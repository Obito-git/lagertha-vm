use crate::token::Span;
use crate::warning::JasmWarning;
use std::ops::Range;

pub(super) enum ParserWarning {
    MissingSuperClass {
        class_name: String,
        class_directive_pos: Span,
        default: &'static str,
    },
}

impl ParserWarning {
    fn message(&self) -> String {
        match self {
            ParserWarning::MissingSuperClass { .. } => "missing super directive".to_string(),
        }
    }

    fn labels(&self) -> Vec<(Range<usize>, String)> {
        match self {
            ParserWarning::MissingSuperClass {
                class_directive_pos,
                class_name,
                ..
            } => vec![(
                class_directive_pos.as_range(),
                format!("Class {} is missing a superclass directive", class_name),
            )],
        }
    }

    fn note(&self) -> String {
        match self {
            ParserWarning::MissingSuperClass { default, .. } => format!(
                "The .super directive is required to specify the superclass. Defaulting to {}.",
                default
            ),
        }
    }

    fn primary_location(&self) -> Range<usize> {
        match self {
            ParserWarning::MissingSuperClass {
                class_directive_pos,
                ..
            } => class_directive_pos.as_range(),
        }
    }
}

impl From<ParserWarning> for JasmWarning {
    fn from(warn: ParserWarning) -> Self {
        JasmWarning::new(
            warn.message(),
            warn.primary_location(),
            warn.labels(),
            warn.note(),
        )
    }
}
