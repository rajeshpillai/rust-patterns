// Broken example — attempting to impl a foreign trait on a foreign
// type. Rust's orphan rule (E0117) forbids it: at least one of the
// trait or the type in `impl Trait for Type` must be defined in the
// current crate, or coherence breaks across downstream consumers.
//
// This file is expected to FAIL to compile.

use std::io::{self, Read};

// Pretend both of these come from separate external crates that we
// do not own. (Represented as sibling modules to avoid needing extra
// dependencies for the demo.)
mod foreign_reader {
    pub struct LegacyReader {
        pub bytes: Vec<u8>,
    }
}

// Tries to implement std::io::Read (foreign trait) for
// foreign_reader::LegacyReader (foreign type). Both live outside the
// current crate, so the orphan rule rejects this impl.
impl Read for foreign_reader::LegacyReader {
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // error[E0117]: only traits defined in the current crate can be
    //               implemented for types defined outside of the crate
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = buf.len().min(self.bytes.len());
        buf[..n].copy_from_slice(&self.bytes[..n]);
        self.bytes.drain(..n);
        Ok(n)
    }
}

fn main() {}
