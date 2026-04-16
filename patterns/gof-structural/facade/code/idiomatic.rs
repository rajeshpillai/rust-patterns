// Facade — one public type that orchestrates a private pipeline.
// Callers see ReportBuilder::generate(path); the four helper modules
// (loader, parser, validator, renderer) stay `pub(crate)` and their
// types never surface in the public API.
//
// This is the shape that library authors default to when they have
// a pipeline the user shouldn't have to assemble themselves.

use std::fmt;

// ---- The public facade -------------------------------------------------

pub struct ReportBuilder {
    strict: bool,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ReportError {
    Load { path: String },
    Parse { line: usize, reason: String },
    Validate { record: usize, reason: String },
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load { path }            => write!(f, "failed to load {path}"),
            Self::Parse { line, reason }   => write!(f, "parse error on line {line}: {reason}"),
            Self::Validate { record, reason } => write!(f, "record {record}: {reason}"),
        }
    }
}
impl std::error::Error for ReportError {}

impl ReportBuilder {
    pub fn new() -> Self { Self { strict: false } }
    pub fn strict(mut self, s: bool) -> Self { self.strict = s; self }

    /// The one call the caller makes. Internally: load → parse → validate → render.
    pub fn generate(&self, path: &str) -> Result<String, ReportError> {
        let bytes    = loader::load_file(path)?;
        let records  = parser::parse_csv(&bytes)?;
        let checked  = validator::check(records, self.strict)?;
        let rendered = renderer::emit(&checked);
        Ok(rendered)
    }
}
impl Default for ReportBuilder { fn default() -> Self { Self::new() } }

// ---- Private subsystems (pub(crate) or private in a real crate) -------

mod loader {
    use super::ReportError;
    // Pretend we read a file; for the demo, hardcode content by path.
    pub fn load_file(path: &str) -> Result<Vec<u8>, ReportError> {
        match path {
            "invoices.csv" =>
                Ok(b"id,amount\n1,100\n2,200\n3,-5\n".to_vec()),
            _ => Err(ReportError::Load { path: path.to_string() }),
        }
    }
}

mod parser {
    use super::ReportError;
    #[derive(Debug)]
    pub struct Record { pub id: u64, pub amount: i64 }

    pub fn parse_csv(bytes: &[u8]) -> Result<Vec<Record>, ReportError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|e| ReportError::Parse { line: 0, reason: e.to_string() })?;
        let mut out = Vec::new();
        for (i, line) in text.lines().enumerate().skip(1) {
            let (id_s, amt_s) = line.split_once(',')
                .ok_or_else(|| ReportError::Parse { line: i, reason: "missing comma".into() })?;
            let id = id_s.parse::<u64>()
                .map_err(|e| ReportError::Parse { line: i, reason: e.to_string() })?;
            let amount = amt_s.parse::<i64>()
                .map_err(|e| ReportError::Parse { line: i, reason: e.to_string() })?;
            out.push(Record { id, amount });
        }
        Ok(out)
    }
}

mod validator {
    use super::{ReportError, parser::Record};
    pub fn check(records: Vec<Record>, strict: bool) -> Result<Vec<Record>, ReportError> {
        for (i, r) in records.iter().enumerate() {
            if strict && r.amount < 0 {
                return Err(ReportError::Validate {
                    record: i,
                    reason: format!("negative amount: {}", r.amount),
                });
            }
        }
        Ok(records)
    }
}

mod renderer {
    use super::parser::Record;
    pub fn emit(records: &[Record]) -> String {
        let mut out = String::from("# Report\n\n");
        out.push_str("| id | amount |\n|---:|-------:|\n");
        for r in records {
            out.push_str(&format!("| {} | {} |\n", r.id, r.amount));
        }
        out
    }
}

fn main() -> Result<(), ReportError> {
    // Non-strict: negative amounts pass through.
    let pdf = ReportBuilder::new().generate("invoices.csv")?;
    println!("{pdf}");

    // Strict: the negative amount fails validation with a typed error.
    let err = ReportBuilder::new()
        .strict(true)
        .generate("invoices.csv")
        .unwrap_err();
    println!("expected strict error: {err}");
    Ok(())
}
