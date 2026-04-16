// Template Method in Rust — a trait with a default method (the
// template) that calls abstract methods (the hooks). Impls fill in
// only the steps that differ per concrete type.
//
// Two Rust shapes:
//   A) Trait with default method — the direct translation. Uses
//      static or dynamic dispatch depending on how you store the
//      impl.
//   B) Generic function over a small "hooks" struct — when the
//      skeleton doesn't need to *be* a trait, just to call a few
//      user-supplied functions.
//
// Option A is the classical Template Method. Option B is often
// better in Rust: closures are cheap, traits are not free.

use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum PipelineError {
    Load { path: String },
    Parse { line: usize, reason: String },
    Validate { reason: String },
}
impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load { path } => write!(f, "failed to load {path}"),
            Self::Parse { line, reason } => write!(f, "line {line}: {reason}"),
            Self::Validate { reason } => write!(f, "validate: {reason}"),
        }
    }
}
impl std::error::Error for PipelineError {}

pub struct Record {
    pub key: String,
    pub value: i64,
}

// ---- A) Trait with default method -----------------------------------

pub trait DataPipeline {
    // The template — every impl inherits this default. Callers who
    // need different ordering should override it; most shouldn't.
    fn run(&self, path: &str) -> Result<String, PipelineError> {
        let bytes    = self.load(path)?;
        let records  = self.parse(&bytes)?;
        let records  = self.validate(records)?;
        Ok(self.emit(&records))
    }

    // The steps each impl MUST provide.
    fn load(&self, path: &str) -> Result<Vec<u8>, PipelineError>;
    fn parse(&self, bytes: &[u8]) -> Result<Vec<Record>, PipelineError>;
    fn emit(&self, records: &[Record]) -> String;

    // A step with a sensible default; subclasses override only if
    // they add extra rules.
    fn validate(&self, records: Vec<Record>) -> Result<Vec<Record>, PipelineError> {
        Ok(records)  // permissive default
    }
}

// Concrete impl #1 — CSV
pub struct CsvPipeline;
impl DataPipeline for CsvPipeline {
    fn load(&self, path: &str) -> Result<Vec<u8>, PipelineError> {
        match path {
            "data.csv" => Ok(b"key,value\nusers,42\norders,7\n".to_vec()),
            _ => Err(PipelineError::Load { path: path.into() }),
        }
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<Record>, PipelineError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|e| PipelineError::Parse { line: 0, reason: e.to_string() })?;
        let mut out = Vec::new();
        for (i, line) in text.lines().enumerate().skip(1) {
            let (k, v) = line.split_once(',')
                .ok_or_else(|| PipelineError::Parse { line: i, reason: "no comma".into() })?;
            let value = v.parse()
                .map_err(|e: std::num::ParseIntError| PipelineError::Parse { line: i, reason: e.to_string() })?;
            out.push(Record { key: k.into(), value });
        }
        Ok(out)
    }
    fn emit(&self, records: &[Record]) -> String {
        records.iter().map(|r| format!("{} = {}", r.key, r.value)).collect::<Vec<_>>().join("\n")
    }
}

// Concrete impl #2 — "Strict CSV" overrides validate() only.
pub struct StrictCsvPipeline;
impl DataPipeline for StrictCsvPipeline {
    fn load(&self, path: &str)       -> Result<Vec<u8>, PipelineError>   { CsvPipeline.load(path) }
    fn parse(&self, bytes: &[u8])    -> Result<Vec<Record>, PipelineError> { CsvPipeline.parse(bytes) }
    fn emit(&self, records: &[Record]) -> String                          { CsvPipeline.emit(records) }
    fn validate(&self, records: Vec<Record>) -> Result<Vec<Record>, PipelineError> {
        if records.iter().any(|r| r.value < 0) {
            return Err(PipelineError::Validate { reason: "negative value".into() });
        }
        Ok(records)
    }
}

// ---- B) Function + closure-hooks (no trait) -------------------------

// When "template" is really just "run these four functions in order",
// a plain function with closure parameters is the lightest form.
pub fn run_with<Load, Parse, Val, Emit>(
    path: &str,
    load: Load,
    parse: Parse,
    validate: Val,
    emit: Emit,
) -> Result<String, PipelineError>
where
    Load:  Fn(&str) -> Result<Vec<u8>, PipelineError>,
    Parse: Fn(&[u8]) -> Result<Vec<Record>, PipelineError>,
    Val:   Fn(Vec<Record>) -> Result<Vec<Record>, PipelineError>,
    Emit:  Fn(&[Record]) -> String,
{
    let bytes = load(path)?;
    let records = parse(&bytes)?;
    let records = validate(records)?;
    Ok(emit(&records))
}

fn main() -> Result<(), PipelineError> {
    println!("--- permissive CSV ---\n{}", CsvPipeline.run("data.csv")?);
    match StrictCsvPipeline.run("data.csv") {
        Ok(s)  => println!("\n--- strict CSV ---\n{s}"),
        Err(e) => println!("\n--- strict CSV ---\nerror: {e}"),
    }
    Ok(())
}
