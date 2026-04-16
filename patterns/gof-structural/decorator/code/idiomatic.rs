// Decorator — a wrapper that preserves a trait while adding behavior.
// Rust's std::io stack is the canonical example: BufReader, GzDecoder,
// and countless middleware types are all "decorators" around any R
// that implements Read.
//
// Two shapes:
//   A) Generic wrapper `Decorator<R>` — monomorphized, zero vtable,
//      inner type fixed at compile time. The default.
//   B) Trait-object wrapper `Decorator` over `Box<dyn Read>` — runtime
//      composition, vtable per call. Use when you need to pick the
//      inner type at runtime (e.g., based on file extension).

use std::io::{self, Read};

// ---- A) Generic LoggingReader ----------------------------------------

pub struct LoggingReader<R: Read> {
    inner: R,
    total: usize,
}

impl<R: Read> LoggingReader<R> {
    pub fn new(inner: R) -> Self { Self { inner, total: 0 } }
    pub fn bytes_read(&self) -> usize { self.total }
}

impl<R: Read> Read for LoggingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.total += n;
        println!("[log] read {n} bytes (cumulative {})", self.total);
        Ok(n)
    }
}

// ---- A') Generic UpperCaseReader — transforms bytes ------------------

pub struct UpperCaseReader<R: Read> {
    inner: R,
}
impl<R: Read> UpperCaseReader<R> {
    pub fn new(inner: R) -> Self { Self { inner } }
}
impl<R: Read> Read for UpperCaseReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        for b in &mut buf[..n] { b.make_ascii_uppercase(); }
        Ok(n)
    }
}

// ---- B) Dyn-erased runtime wrapper -----------------------------------

// When the inner type is chosen at runtime, the stack looks like
// `Box<dyn Read>`. The decorator wraps that instead of a generic R.
pub struct TimedReader {
    inner: Box<dyn Read>,
    started_at: std::time::Instant,
    total: usize,
}

impl TimedReader {
    pub fn new(inner: Box<dyn Read>) -> Self {
        Self { inner, started_at: std::time::Instant::now(), total: 0 }
    }
}

impl Read for TimedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.total += n;
        Ok(n)
    }
}

impl Drop for TimedReader {
    fn drop(&mut self) {
        let dur = self.started_at.elapsed();
        println!("[timed] total {} bytes in {:.2?}", self.total, dur);
    }
}

fn main() -> io::Result<()> {
    // Static composition — the whole stack is monomorphized.
    let data = b"hello, world\n".to_vec();
    let src = std::io::Cursor::new(data);
    let mut stacked = LoggingReader::new(UpperCaseReader::new(src));

    let mut out = String::new();
    stacked.read_to_string(&mut out)?;
    println!("static stack output: {out:?}");
    println!("static stack total:  {}", stacked.bytes_read());

    // Dynamic composition — pick the inner type at runtime.
    let choice: &str = "memory";
    let inner: Box<dyn Read> = match choice {
        "memory" => Box::new(std::io::Cursor::new(b"dyn world".to_vec())),
        "empty"  => Box::new(std::io::empty()),
        _        => Box::new(std::io::Cursor::new(Vec::new())),
    };
    let mut timed = TimedReader::new(inner);
    let mut dyn_out = String::new();
    timed.read_to_string(&mut dyn_out)?;
    println!("dynamic: {dyn_out:?}");
    // TimedReader's Drop prints the timing summary on scope exit.
    Ok(())
}
