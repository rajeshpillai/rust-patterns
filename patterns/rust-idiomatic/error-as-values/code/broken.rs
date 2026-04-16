// Broken example — two ways people *almost* get error handling right,
// then find out the compiler won't let them.
//
// This file is expected to FAIL to compile.

use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ConfigError {
    Missing,
    InvalidPort,
}

pub fn read_file(_p: &str) -> Result<String, io::Error> {
    Err(io::Error::new(io::ErrorKind::NotFound, "nope"))
}

pub fn parse_port(_s: &str) -> Result<u16, ParseIntError> {
    "abc".parse::<u16>()
}

// Mistake #1 — try to `?` a foreign error (`io::Error`) through a
// function returning our own `ConfigError`. Without `impl From<io::Error>
// for ConfigError`, the `?` operator cannot convert, so this fails
// with E0277 (no From impl) or similar.
pub fn load(path: &str) -> Result<String, ConfigError> {
    let body = read_file(path)?;
    //                          ^ error[E0277]: `?` couldn't convert
    //                            the error to `ConfigError`
    //                            the trait `From<io::Error>` is not
    //                            implemented for `ConfigError`
    Ok(body)
}

// Mistake #2 — try to `?` a `ParseIntError` through the same function
// without a `From<ParseIntError> for ConfigError` impl. Same failure.
pub fn port(s: &str) -> Result<u16, ConfigError> {
    let p = parse_port(s)?;
    //                   ^ error[E0277]: `?` couldn't convert the error
    //                     the trait `From<ParseIntError>` is not
    //                     implemented for `ConfigError`
    Ok(p)
}

fn main() {
    let _ = load("cfg");
    let _ = port("8080");
}
