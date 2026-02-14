use ariadne::{Color, Label, Report, ReportKind, Source};
use std::fmt::Debug;
use std::ops::Range;

pub trait Diagnostic: Debug {
    fn message(&self) -> String;
    fn primary_location(&self) -> Range<usize>;
    fn labels(&self) -> Vec<(Range<usize>, String)>;
    fn note(&self) -> Option<String>;
    fn severity(&self) -> Severity;

    fn print(&self, filename: &str, source_code: &str) {
        let range = self.primary_location();
        let filename_owned = filename.to_string();
        let mut report = Report::build(
            self.severity().into(),
            (filename_owned.clone(), range.clone()),
        )
        .with_message(self.message());

        if let Some(note) = self.note() {
            report = report.with_note(note);
        }

        for (label_range, label_message) in self.labels() {
            report = report.with_label(
                Label::new((filename_owned.clone(), label_range))
                    .with_message(label_message)
                    .with_color(self.severity().color()),
            );
        }

        report
            .finish()
            .eprint((filename_owned, Source::from(source_code)))
            .unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    fn color(&self) -> Color {
        match self {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
        }
    }
}

impl From<Severity> for ReportKind<'_> {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Error => ReportKind::Error,
            Severity::Warning => ReportKind::Warning,
        }
    }
}

pub enum JasmError {
    Diagnostic(Box<dyn Diagnostic>),
    Internal(String),
}

impl Debug for JasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JasmError::Diagnostic(_) => write!(f, "JasmError::Diagnostic(<diagnostic>)"),
            JasmError::Internal(msg) => write!(f, "JasmError::Internal({})", msg),
        }
    }
}

impl JasmError {
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
                diag.print(filename, source_code);
            }
            JasmError::Internal(msg) => {
                eprintln!("{}", Self::format_internal_error(msg));
            }
        }
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
