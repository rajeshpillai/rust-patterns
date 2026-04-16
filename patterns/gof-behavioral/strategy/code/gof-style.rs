// Classical Strategy — Context holds a boxed trait object and
// delegates to it. This is the textbook Rust port. It works fine and
// is the right choice when the set of strategies is truly open
// (plugins, user-supplied, configured at runtime).
//
// For a closed set known at compile time, see code/idiomatic.rs for
// the generic and enum forms.

pub trait Compress {
    fn compress(&self, input: &[u8]) -> Vec<u8>;
    fn name(&self) -> &'static str;
}

pub struct GzipStrategy;
pub struct ZstdStrategy;
pub struct NoopStrategy;

impl Compress for GzipStrategy {
    fn compress(&self, input: &[u8]) -> Vec<u8> {
        // pretend we gzipped it
        let mut out = Vec::with_capacity(input.len() + 4);
        out.extend_from_slice(b"gz:");
        out.extend_from_slice(input);
        out
    }
    fn name(&self) -> &'static str { "gzip" }
}

impl Compress for ZstdStrategy {
    fn compress(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len() + 5);
        out.extend_from_slice(b"zstd:");
        out.extend_from_slice(input);
        out
    }
    fn name(&self) -> &'static str { "zstd" }
}

impl Compress for NoopStrategy {
    fn compress(&self, input: &[u8]) -> Vec<u8> { input.to_vec() }
    fn name(&self) -> &'static str { "noop" }
}

pub struct Uploader {
    strategy: Box<dyn Compress>,
}

impl Uploader {
    pub fn new(strategy: Box<dyn Compress>) -> Self { Self { strategy } }
    pub fn upload(&self, body: &[u8]) {
        let payload = self.strategy.compress(body);
        println!("upload {} bytes with {}", payload.len(), self.strategy.name());
    }
    pub fn set_strategy(&mut self, s: Box<dyn Compress>) { self.strategy = s; }
}

fn main() {
    let mut up = Uploader::new(Box::new(GzipStrategy));
    up.upload(b"hello world");
    up.set_strategy(Box::new(ZstdStrategy));
    up.upload(b"hello world");
}
