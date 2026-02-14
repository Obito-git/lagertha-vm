use ariadne::{Color, Label, Report, ReportKind, Source};
use std::ops::Range;

#[derive(Debug)]
pub enum JasmError {
    Diagnostic(JasmDiagnostic),
    Internal(String),
}

impl JasmError {
    fn print_diagnostic_error(filename: &str, source_code: &str, err: JasmDiagnostic) {
        let range = err.primary_location();
        let mut report =
            Report::build(ReportKind::Error, (filename, range.clone())).with_message(err.message());

        if let Some(note) = err.note() {
            report = report.with_note(note);
        }

        for (label_range, label_message) in err.labels() {
            report = report.with_label(
                Label::new((filename, label_range.clone()))
                    .with_message(label_message)
                    .with_color(Color::Red),
            );
        }

        report
            .finish()
            .eprint((filename, Source::from(source_code)))
            .unwrap();
    }

    fn format_internal_error(message: &str) -> String {
        [
            "",
            "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
            "!            INTERNAL ASSEMBLER ERROR             !",
            "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
            "",
            "This is a bug in the jasm assembler.",
            "Please report it at: https://github.com/Obito-git/lagertha-vm/issues",
            "",
            &format!("Details: {message}"),
        ]
        .join("\n")
    }

    pub fn print(&self, filename: &str, source_code: &str) {
        match self {
            JasmError::Diagnostic(diag) => {
                Self::print_diagnostic_error(filename, source_code, diag.clone())
            }
            JasmError::Internal(msg) => {
                eprintln!("{}", Self::format_internal_error(msg));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JasmDiagnostic {
    message: String,
    primary_location: Range<usize>,
    label: Vec<(Range<usize>, String)>,
    note: Option<String>,
}

impl JasmDiagnostic {
    pub fn new(
        message: impl Into<String>,
        primary_location: Range<usize>,
        label: Vec<(Range<usize>, String)>,
        note: Option<String>,
    ) -> Self {
        Self {
            message: message.into(),
            primary_location,
            note,
            label,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn primary_location(&self) -> &Range<usize> {
        &self.primary_location
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    pub fn labels(&self) -> &Vec<(Range<usize>, String)> {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_internal_error() {
        let output = JasmError::format_internal_error("unexpected state in parser");

        insta::assert_snapshot!(output);
    }
}
