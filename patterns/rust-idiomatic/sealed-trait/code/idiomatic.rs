// Sealed Trait — a public trait only your crate's types can implement.
//
// Why seal?
//   * You want to ship `pub trait Format` as a public type-level API
//     that consumers can use as a bound, without letting them add
//     implementations you'd then have to support forever.
//   * You want to refactor internal types freely — adding methods,
//     changing defaults — without breaking downstream impls that
//     don't exist.
//   * You want the compiler to enforce "this trait is closed" so
//     `match kind { Format::Json => ..., Format::Yaml => ... }`
//     reasoning holds.
//
// Mechanism: a *supertrait* the downstream cannot name. Put it in a
// private module, require it on the public trait, and only implement
// it for types inside the crate. Downstream impls of the public trait
// fail to compile because they can't satisfy the supertrait bound.

// ---- The seal -------------------------------------------------------

mod private {
    // Public *inside* the crate, but the module `private` itself is
    // not `pub`, so downstream users cannot write `sealed_trait::
    // private::Sealed`. They cannot implement it, therefore they
    // cannot implement anything that requires it.
    pub trait Sealed {}
}

// ---- The public trait, sealed --------------------------------------

pub trait Format: private::Sealed {
    fn extension(&self) -> &'static str;
    fn render(&self, data: &[(&str, i64)]) -> String;
}

// ---- Concrete implementations (only inside this crate) -------------

pub struct Json;
pub struct Yaml;

impl private::Sealed for Json {}
impl private::Sealed for Yaml {}

impl Format for Json {
    fn extension(&self) -> &'static str { "json" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        let body: Vec<String> = data
            .iter()
            .map(|(k, v)| format!("  {:?}: {}", k, v))
            .collect();
        format!("{{\n{}\n}}", body.join(",\n"))
    }
}

impl Format for Yaml {
    fn extension(&self) -> &'static str { "yaml" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        data.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ---- Consumer code (downstream-style) --------------------------------

// Consumers of this crate can take `F: Format` as a generic bound. They
// can CALL the trait methods, but they cannot implement Format for
// their own types.
pub fn write_report<F: Format>(fmt: &F, data: &[(&str, i64)]) -> String {
    let body = fmt.render(data);
    format!("report.{}\n---\n{body}", fmt.extension())
}

fn main() {
    let data = &[("users", 42), ("orders", 7)];
    println!("{}", write_report(&Json, data));
    println!();
    println!("{}", write_report(&Yaml, data));
}
