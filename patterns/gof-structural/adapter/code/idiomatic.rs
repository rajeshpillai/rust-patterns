// Adapter — a tiny wrapper that makes a foreign type satisfy a
// target trait. Here we adapt a made-up `LegacyReader` so it can be
// used anywhere a `std::io::Read` is expected.
//
// Three moves:
//   1. A newtype (`LegacyAdapter`) owned by OUR module, satisfying
//      the orphan rule.
//   2. `impl Read for LegacyAdapter` — the trait we care about,
//      driven by the foreign API's methods.
//   3. `From<LegacyReader> for LegacyAdapter` so construction is
//      ergonomic: `let a: LegacyAdapter = reader.into();`.

use std::io::{self, Read};

// ---- The "foreign" type ------------------------------------------------
// Pretend this comes from another crate. It does its own thing; we can't
// modify it. Crucially, we CAN'T write `impl Read for LegacyReader` here
// — the orphan rule (E0117) forbids impls of foreign traits on foreign
// types.

pub struct LegacyReader {
    chunks: Vec<Vec<u8>>,
}

impl LegacyReader {
    pub fn new(chunks: Vec<Vec<u8>>) -> Self { Self { chunks } }
    pub fn read_chunk(&mut self) -> Option<Vec<u8>> {
        if self.chunks.is_empty() { None } else { Some(self.chunks.remove(0)) }
    }
}

// ---- The adapter --------------------------------------------------------

pub struct LegacyAdapter {
    inner: LegacyReader,
    // Leftover bytes that didn't fit in the caller's `buf` from the
    // previous read_chunk() pull.
    leftover: Vec<u8>,
}

impl From<LegacyReader> for LegacyAdapter {
    fn from(inner: LegacyReader) -> Self {
        Self { inner, leftover: Vec::new() }
    }
}

impl Read for LegacyAdapter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If we still have leftovers from a previous chunk, drain
        // those first. Otherwise pull a fresh chunk from the legacy
        // reader. Return 0 to signal EOF.
        if self.leftover.is_empty() {
            match self.inner.read_chunk() {
                Some(chunk) => self.leftover = chunk,
                None => return Ok(0),
            }
        }
        let n = buf.len().min(self.leftover.len());
        buf[..n].copy_from_slice(&self.leftover[..n]);
        self.leftover.drain(..n);
        Ok(n)
    }
}

fn main() -> io::Result<()> {
    // Consumers of our adapter don't know or care about LegacyReader —
    // they see a std::io::Read. Any helper that expects `Read`
    // (serde_json::from_reader, BufReader::new, bincode, etc.) now
    // works with LegacyReader data via the adapter.
    let legacy = LegacyReader::new(vec![
        b"hello ".to_vec(),
        b"world\n".to_vec(),
        b"patterns".to_vec(),
    ]);
    let mut adapter: LegacyAdapter = legacy.into();

    let mut out = String::new();
    adapter.read_to_string(&mut out)?;
    println!("{out}");

    Ok(())
}
