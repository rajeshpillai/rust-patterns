// Flyweight in Rust — the built-in version is `Arc<T>` for shared
// immutable state. Add a simple pool (HashMap<K, Arc<T>>) to intern
// unique values and the pattern is complete.
//
// The example models a page of glyphs for a text renderer. The heavy
// `FontData` (imagine a loaded TTF buffer, megabytes in size) is
// shared across thousands of glyph instances via Arc; only the
// lightweight per-instance data (char, position, font handle) is
// duplicated.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ---- The "heavy" shared payload -------------------------------------

#[derive(Debug)]
pub struct FontData {
    pub name: String,
    pub size: u16,
    /// In reality this would be the whole TTF buffer. A single
    /// Vec<u8> per font stands in for that weight here.
    pub payload: Vec<u8>,
}

impl FontData {
    fn new(name: &str, size: u16) -> Self {
        Self {
            name: name.into(),
            size,
            // pretend we loaded 1 MiB of TTF bytes
            payload: vec![0_u8; 1024 * 1024],
        }
    }
}

// ---- The flyweight pool ---------------------------------------------

pub struct FontPool {
    inner: Mutex<HashMap<(String, u16), Arc<FontData>>>,
}

impl FontPool {
    pub fn new() -> Self { Self { inner: Mutex::new(HashMap::new()) } }

    /// `get` interns by (name, size). Callers with the same pair get
    /// the same `Arc<FontData>`; the underlying buffer is allocated
    /// exactly once per unique key.
    pub fn get(&self, name: &str, size: u16) -> Arc<FontData> {
        let mut map = self.inner.lock().expect("font pool mutex poisoned");
        if let Some(f) = map.get(&(name.into(), size)) {
            return Arc::clone(f);
        }
        let f = Arc::new(FontData::new(name, size));
        map.insert((name.into(), size), Arc::clone(&f));
        f
    }
}

impl Default for FontPool { fn default() -> Self { Self::new() } }

// ---- The light per-instance object ----------------------------------

pub struct Glyph {
    pub ch: char,
    pub x:  u16,
    pub y:  u16,
    pub font: Arc<FontData>,
}

impl Glyph {
    pub fn new(ch: char, x: u16, y: u16, font: Arc<FontData>) -> Self {
        Self { ch, x, y, font }
    }
}

fn main() {
    let pool = FontPool::new();

    // "Render" a million glyphs across 3 different font/size pairs.
    let mut glyphs = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000_u32 {
        let (name, size) = match i % 3 {
            0 => ("Inter", 12),
            1 => ("Inter", 16),
            _ => ("Mono",  12),
        };
        let font = pool.get(name, size);
        let ch = char::from_u32(b'a' as u32 + (i as u32 % 26)).unwrap_or('?');
        glyphs.push(Glyph::new(ch, (i % 1024) as u16, (i / 1024) as u16, font));
    }

    println!("glyphs: {}", glyphs.len());
    println!(
        "unique FontData buffers in pool: {}",
        pool.inner.lock().unwrap().len()
    );

    // Each FontData is allocated exactly once; Glyph instances share it.
    // Without flyweight: 1,000,000 × 1 MiB = 1 TB.
    // With flyweight:    3 × 1 MiB + (1,000,000 × small struct) ≈ 3 MiB + ~30 MiB.
    let total_arc_count: usize = glyphs.iter().map(|g| Arc::strong_count(&g.font)).max().unwrap_or(0);
    println!("max strong_count on one FontData: {total_arc_count}");
}
