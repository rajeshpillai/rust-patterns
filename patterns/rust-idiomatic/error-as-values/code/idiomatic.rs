// Error-as-Values — the full story:
//
//   * A typed error enum for the library ("what went wrong").
//   * `#[non_exhaustive]` so adding variants isn't a breaking change.
//   * `impl Display` for the user-facing message.
//   * `impl std::error::Error` + `source()` so errors chain.
//   * `impl From<InnerErr> for OuterErr` so `?` propagates cleanly.
//
// No dependencies — the same shape that `thiserror` generates for
// you, written by hand so you can see what's happening. In real
// code, prefer `thiserror` for libraries and `anyhow` for binaries.

use std::fmt;
use std::io;
use std::num::ParseIntError;

// ---- The error type --------------------------------------------------

#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigError {
    /// `expected` field was missing from the config file.
    Missing { field: &'static str },
    /// `port` parsed but was outside the valid range.
    InvalidPort { value: u32 },
    /// Config file could not be read — wraps the underlying IO error.
    Io(io::Error),
    /// A numeric field failed to parse — wraps the parse error.
    Parse { field: &'static str, source: ParseIntError },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { field }      => write!(f, "required field missing: {field}"),
            Self::InvalidPort { value }  => write!(f, "port {value} is outside 1..=65535"),
            Self::Io(_)                  => f.write_str("failed to read config file"),
            Self::Parse { field, .. }    => write!(f, "failed to parse field {field:?}"),
        }
    }
}

impl std::error::Error for ConfigError {
    // `source()` is how errors *chain*. `anyhow` and Rust's default
    // `{:#}` formatting walk the chain and print every link.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e)           => Some(e),
            Self::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ---- From impls so `?` converts automatically ------------------------

// `io::Error -> ConfigError`: any function returning `Result<_, io::Error>`
// can be `?`'d inside a function returning `Result<_, ConfigError>`.
impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self { Self::Io(e) }
}

// ---- Using the error type --------------------------------------------

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

pub fn parse_config(lines: &[&str]) -> Result<Config, ConfigError> {
    let mut url = None;
    let mut port = None;

    for line in lines {
        let (k, v) = line.split_once('=')
            .ok_or(ConfigError::Missing { field: "separator" })?;
        match k.trim() {
            "database_url" => url = Some(v.trim().to_string()),
            "port" => {
                let raw: u32 = v.trim().parse()
                    // `map_err` is how we convert foreign errors into
                    // our own when the `From` impl doesn't fit (e.g.,
                    // because we want to attach a field name).
                    .map_err(|source| ConfigError::Parse { field: "port", source })?;
                if !(1..=65535).contains(&raw) {
                    return Err(ConfigError::InvalidPort { value: raw });
                }
                port = Some(raw as u16);
            }
            _ => { /* ignore unknown fields for this toy example */ }
        }
    }

    Ok(Config {
        database_url: url.ok_or(ConfigError::Missing { field: "database_url" })?,
        port: port.ok_or(ConfigError::Missing { field: "port" })?,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Happy path
    let cfg = parse_config(&[
        "database_url = postgres://localhost/dev",
        "port = 8080",
    ])?;
    println!("parsed: {cfg:?}");

    // Missing field
    let err = parse_config(&["port = 8080"]).unwrap_err();
    println!("display:   {err}");
    println!("debug:     {err:?}");
    // Pretty-print the whole chain, mimicking anyhow's {:#}.
    let mut cur: Option<&(dyn std::error::Error + 'static)> = Some(&err);
    while let Some(e) = cur {
        println!("chain-link: {e}");
        cur = e.source();
    }

    // Parse error — demonstrates wrapping a foreign error type.
    let err = parse_config(&[
        "database_url = ignored",
        "port = abc",
    ]).unwrap_err();
    println!("{err}");

    Ok(())
}
