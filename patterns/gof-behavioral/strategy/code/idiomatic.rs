// Three idiomatic Rust shapes for Strategy — pick the one that
// matches the set of strategies you need:
//
//   A) Generic parameter — fastest, fully inlined, binary grows per
//      instantiation. Best when one call site uses one strategy.
//
//   B) Closure — no trait needed, captures local state, good for
//      one-shot hooks (e.g. sort_by).
//
//   C) Enum + match — closed set of strategies known at compile
//      time. Exhaustive match forces you to handle new variants.

// ---- A) Generic parameter --------------------------------------------

pub trait Compress {
    fn compress(&self, input: &[u8]) -> Vec<u8>;
    fn name(&self) -> &'static str;
}

pub struct GzipStrategy;
impl Compress for GzipStrategy {
    fn compress(&self, input: &[u8]) -> Vec<u8> {
        let mut out = b"gz:".to_vec(); out.extend_from_slice(input); out
    }
    fn name(&self) -> &'static str { "gzip" }
}

// Generic over S: compiles to a *separate function* per instantiation.
// Zero vtable, zero indirection, fully inlined at the call site.
pub fn upload_static<S: Compress>(strategy: &S, body: &[u8]) {
    let payload = strategy.compress(body);
    println!("[static] {} bytes via {}", payload.len(), strategy.name());
}

// ---- B) Closure -------------------------------------------------------

// No trait needed. Callers supply compression inline.
pub fn upload_with(compress: impl Fn(&[u8]) -> Vec<u8>, body: &[u8]) {
    let payload = compress(body);
    println!("[closure] {} bytes", payload.len());
}

// ---- C) Enum + match --------------------------------------------------

#[derive(Debug)]
pub enum CompressionKind {
    Gzip,
    Zstd,
    Noop,
}

impl CompressionKind {
    pub fn compress(&self, input: &[u8]) -> Vec<u8> {
        match self {
            Self::Gzip => { let mut o = b"gz:".to_vec();   o.extend_from_slice(input); o }
            Self::Zstd => { let mut o = b"zstd:".to_vec(); o.extend_from_slice(input); o }
            Self::Noop => input.to_vec(),
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gzip => "gzip",
            Self::Zstd => "zstd",
            Self::Noop => "noop",
        }
    }
}

pub fn upload_enum(kind: &CompressionKind, body: &[u8]) {
    let payload = kind.compress(body);
    println!("[enum] {} bytes via {}", payload.len(), kind.name());
}

// -----------------------------------------------------------------------

fn main() {
    let body = b"hello world";

    // A) generic
    upload_static(&GzipStrategy, body);

    // B) closure with captured state
    let suffix = b":tagged";
    upload_with(
        |input| {
            let mut v = input.to_vec();
            v.extend_from_slice(suffix);
            v
        },
        body,
    );

    // C) enum — exhaustive, closed set
    for kind in [CompressionKind::Gzip, CompressionKind::Zstd, CompressionKind::Noop] {
        upload_enum(&kind, body);
    }
}
