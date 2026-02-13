use ariadne::{Color, Label, Report, ReportKind, Source};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JasmWarning {
    message: String,
    range: Option<Range<usize>>,
    note: Option<String>,
    label: Option<String>,
}

impl JasmWarning {
    pub fn new(
        message: impl Into<String>,
        range: Option<Range<usize>>,
        note: Option<String>,
        label: Option<String>,
    ) -> Self {
        Self {
            message: message.into(),
            range,
            note,
            label,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn range(&self) -> Option<&Range<usize>> {
        self.range.as_ref()
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
