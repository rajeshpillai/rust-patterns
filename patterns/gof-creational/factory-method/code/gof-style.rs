// Classical GoF Factory Method — Rust port with a trait + associated
// type that plays the role of the "abstract Creator class."
// Subclasses become concrete impls.
//
// This works, but feels heavier than it needs to in Rust. Compare
// with code/idiomatic.rs for the enum and generic forms.

pub trait Formatter {
    fn extension(&self) -> &'static str;
    fn render(&self, data: &[(&str, i64)]) -> String;
}

/// Abstract "Creator" — each impl names the Product it produces via
/// an associated type and returns one from the factory method.
pub trait Creator {
    type Product: Formatter;
    fn create(&self) -> Self::Product;

    /// `do_work` is the "Creator uses Product" part of the GoF
    /// presentation — demonstrates why the factory exists.
    fn do_work(&self, data: &[(&str, i64)]) -> String {
        let product = self.create();
        let body = product.render(data);
        format!("[{}]\n{body}", product.extension())
    }
}

pub struct Json;
impl Formatter for Json {
    fn extension(&self) -> &'static str { "json" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        let mut parts: Vec<String> = data.iter().map(|(k, v)| format!("  \"{k}\": {v}")).collect();
        parts.insert(0, "{".into());
        parts.push("}".into());
        parts.join("\n")
    }
}

pub struct Yaml;
impl Formatter for Yaml {
    fn extension(&self) -> &'static str { "yaml" }
    fn render(&self, data: &[(&str, i64)]) -> String {
        data.iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join("\n")
    }
}

/// Concrete Creator for Json.
pub struct JsonCreator;
impl Creator for JsonCreator {
    type Product = Json;
    fn create(&self) -> Json { Json }
}

/// Concrete Creator for Yaml.
pub struct YamlCreator;
impl Creator for YamlCreator {
    type Product = Yaml;
    fn create(&self) -> Yaml { Yaml }
}

fn main() {
    let data = &[("users", 42), ("orders", 7)];
    println!("{}", JsonCreator.do_work(data));
    println!();
    println!("{}", YamlCreator.do_work(data));
}
