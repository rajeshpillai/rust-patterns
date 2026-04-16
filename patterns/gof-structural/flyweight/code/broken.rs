// Broken example — two common Flyweight failures.
// This file is expected to FAIL to compile.
//
//   1. Store the "shared" FontData BY VALUE in each Glyph. Every
//      glyph owns its own copy; the whole "sharing" property is lost.
//      This compiles — and that's the bug. Memory blows up linearly.
//
//   2. Use `&FontData` with a lifetime that can't outlive the call
//      site. The glyphs can't be returned or stored in a long-lived
//      Vec because the reference outlives its source.

pub struct FontData {
    pub name: String,
    pub payload: Vec<u8>,
}

impl FontData {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), payload: vec![0_u8; 1024 * 1024] }
    }
}

// Mistake #1 — stores the heavy buffer BY VALUE. Cloning fonts per
// glyph, not sharing them. This compiles, which is the trap.
#[derive(Clone)]
pub struct OwnedGlyph {
    pub ch: char,
    pub font: FontData,     // not Arc, not &, just owned duplicate
}

// Mistake #2 — tries to share a reference but gets bitten by
// lifetimes when returning the Vec.
pub fn build_glyphs<'a>(count: usize) -> Vec<BorrowedGlyph<'a>> {
    let font = FontData::new("Inter");
    //  ^^^^ error[E0515]: cannot return value referencing local
    //                     variable `font`
    let font_ref = &font;
    (0..count).map(|i| BorrowedGlyph {
        ch: '?',
        font: font_ref,
    }).collect()
}

pub struct BorrowedGlyph<'a> {
    pub ch: char,
    pub font: &'a FontData,
}

fn main() {
    // Mistake #1 compiles but is the wrong design.
    let _millions: Vec<OwnedGlyph> = (0..1_000_000).map(|_| OwnedGlyph {
        ch: 'x',
        font: FontData::new("Inter"),
    }).collect();
    // The program above allocates ~1 TB. Real mistake in production.

    let _ = build_glyphs(10);
}
