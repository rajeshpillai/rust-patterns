// From / Into / TryFrom / TryInto — Rust's standard conversion
// contract. The important rules:
//
//   1. If a conversion CANNOT fail, impl `From<U> for T`. `Into<T>
//      for U` comes automatically.
//
//   2. If a conversion CAN fail, impl `TryFrom<U> for T` with a typed
//      `Error`. `TryInto<T> for U` comes automatically.
//
//   3. Always impl `From`/`TryFrom`, not `Into`/`TryInto`. The direction
//      matters: `From` is the canonical one, `Into` is the reverse
//      lookup the compiler synthesises.
//
//   4. The `?` operator calls `.into()` on the Err value. That's why
//      "`?` converts errors automatically" — it's dispatching through
//      your `From<InnerErr> for OuterErr` impl.

use std::fmt;
use std::num::ParseIntError;

// ---- Case A: infallible From — newtype over &str ---------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

// `impl From<&str> for Name` — cannot fail, always produces a Name.
// This makes `let n: Name = "Rajesh".into();` work at every call site.
impl From<&str> for Name {
    fn from(s: &str) -> Self { Name(s.to_string()) }
}

// And the reverse: Name -> String is infallible, so From works there too.
impl From<Name> for String {
    fn from(n: Name) -> String { n.0 }
}

// ---- Case B: fallible TryFrom — port number validation --------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Port(u16);

#[derive(Debug)]
pub struct OutOfRange { pub value: i64 }

impl fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "port {} is outside 1..=65535", self.value)
    }
}
impl std::error::Error for OutOfRange {}

impl TryFrom<i64> for Port {
    type Error = OutOfRange;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if (1..=65535).contains(&v) { Ok(Port(v as u16)) }
        else                         { Err(OutOfRange { value: v }) }
    }
}

// ---- Case C: From for errors, enabling `?` through frame boundaries --

#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigError {
    Missing { field: &'static str },
    Parse { field: &'static str, source: ParseIntError },
    InvalidPort { value: i64 },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { field } => write!(f, "missing: {field}"),
            Self::Parse { field, .. } => write!(f, "parse error in {field}"),
            Self::InvalidPort { value } => write!(f, "bad port {value}"),
        }
    }
}
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}

// Now this From impl lets `?` propagate OutOfRange through a function
// that returns ConfigError — zero map_err boilerplate.
impl From<OutOfRange> for ConfigError {
    fn from(e: OutOfRange) -> Self {
        ConfigError::InvalidPort { value: e.value }
    }
}

pub fn load_port_from(s: &str) -> Result<Port, ConfigError> {
    // parse returns ParseIntError. No From for that yet, so map_err it.
    let raw = s.parse::<i64>()
        .map_err(|source| ConfigError::Parse { field: "port", source })?;
    // Port::try_from returns OutOfRange. From<OutOfRange> for ConfigError
    // exists, so `?` handles the conversion with no extra code.
    let port: Port = raw.try_into()?;
    Ok(port)
}

fn main() {
    // A) Infallible
    let n: Name = "Rajesh".into();
    let s: String = n.clone().into();
    println!("{:?} / {:?}", n, s);

    // B) Fallible
    let ok: Port = 8080_i64.try_into().unwrap();
    println!("ok: {ok:?}");
    let bad: Result<Port, OutOfRange> = 70000_i64.try_into();
    println!("bad: {bad:?}");

    // C) `?` across error-type boundaries
    println!("load_port_from(\"8080\")   -> {:?}", load_port_from("8080"));
    println!("load_port_from(\"80808\")  -> {:?}", load_port_from("80808"));
    println!("load_port_from(\"abc\")    -> {:?}", load_port_from("abc"));
}
