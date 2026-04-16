// Factory Method in idiomatic Rust — three shapes.
//
//   A) Enum-tag dispatch at runtime (`Box<dyn Trait>`) — when the
//      concrete type is chosen by user input, config, or file ext.
//   B) Generic parameter — when the caller picks the concrete type
//      statically. Monomorphized, zero vtable.
//   C) Closure constructor — when the caller has the construction
//      logic and the factory just calls it.
//
// Most "Factory Method" problems in Rust are best solved by (A) or
// (B). Reach for the classical trait + Creator hierarchy only when
// neither fits — which is rare.

// ---- Common Product trait --------------------------------------------

pub trait Formatter {
    fn extension(&self) -> &'static str;
    fn render(&self, data: &[(&str, i64)]) -> String;
}

pub struct Json;
pub struct Yaml;
pub struct Toml;

impl Formatter for Json {
    fn extension(&self) -> &'static str { "json" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        let mut s = String::from("{\n");
        for (i, (k, v)) in data.iter().enumerate() {
            if i > 0 { s.push_str(",\n"); }
            s.push_str(&format!("  \"{k}\": {v}"));
        }
        s.push_str("\n}");
        s
    }
}
impl Formatter for Yaml {
    fn extension(&self) -> &'static str { "yaml" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        data.iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join("\n")
    }
}
impl Formatter for Toml {
    fn extension(&self) -> &'static str { "toml" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        data.iter().map(|(k, v)| format!("{k} = {v}")).collect::<Vec<_>>().join("\n")
    }
}

// ---- A) Enum-tag dispatch --------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum FormatKind { Json, Yaml, Toml }

impl FormatKind {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }
}

/// Factory function — the one most people call "factory method" in Rust.
/// Returns a trait object because the concrete type is chosen at runtime.
pub fn make_formatter(kind: FormatKind) -> Box<dyn Formatter> {
    match kind {
        FormatKind::Json => Box::new(Json),
        FormatKind::Yaml => Box::new(Yaml),
        FormatKind::Toml => Box::new(Toml),
    }
}

// ---- B) Generic parameter --------------------------------------------

/// When the caller names the concrete type, static dispatch is free.
/// `F: Formatter + Default` lets us use `F::default()` as the "factory
/// method". For non-Default types, add a trait bound that exposes a
/// `fn new()` (see the GoF-style creator trait in gof-style.rs).
pub fn make_static<F: Formatter + Default>() -> F {
    F::default()
}

// Needed because we declared Json/Yaml/Toml without fields but still
// want `::default()` to work.
impl Default for Json { fn default() -> Self { Json } }
impl Default for Yaml { fn default() -> Self { Yaml } }
impl Default for Toml { fn default() -> Self { Toml } }

// ---- C) Closure constructor ------------------------------------------

/// "Factory" becomes a closure the caller supplies. No trait, no
/// hierarchy, full control of construction.
pub fn with_ctor<P, F: Fn() -> P>(ctor: F) -> P {
    ctor()
}

fn main() {
    let data = &[("users", 42), ("orders", 7)];

    // A) Enum-tag — the runtime factory.
    for ext in ["json", "yml", "toml", "xml"] {
        match FormatKind::from_extension(ext) {
            Some(kind) => {
                let f = make_formatter(kind);
                println!("[{}]\n{}\n", f.extension(), f.render(data));
            }
            None => println!("(unknown extension: {ext})\n"),
        }
    }

    // B) Generic — caller picks the concrete type.
    let f: Yaml = make_static();
    println!("generic:\n{}", f.render(data));

    // C) Closure — caller supplies construction.
    let j = with_ctor(|| Json);
    println!("\nclosure:\n{}", j.render(data));
}
