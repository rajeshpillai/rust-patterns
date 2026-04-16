// Broken example — two common traps when composing decorators.
// This file is expected to FAIL to compile.
//
//   1. Storing `dyn Read` by value in a decorator struct. Trait
//      objects are unsized; they must live behind a pointer
//      (Box, &, Arc) and the field must reflect that.
//
//   2. Forgetting to forward Read correctly — returning the wrong
//      byte count from `read()` leads to runtime bugs; but a subtler
//      compile-time bug is dropping the `?` on the inner call.

use std::io::{self, Read};

// Mistake #1: decorator holds `dyn Read` inline — unsized.
pub struct BadLogger {
    inner: dyn Read,
    //     ^^^^^^^^ error[E0277]: the size for values of type
    //              `(dyn std::io::Read + 'static)` cannot be known
    //              at compilation time
}

// The fix is either:
//   (a) Generic over R: `pub struct BadLogger<R: Read> { inner: R }`
//   (b) Boxed:           `pub struct BadLogger { inner: Box<dyn Read> }`

// Mistake #2: forwarding Read but dropping the return value.
pub struct NoOpDecorator<R: Read> { inner: R }

impl<R: Read> Read for NoOpDecorator<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf);
        //              ^^^^^^^^ warning: unused `Result` — not a hard
        //                       error, but a clippy -D warnings build
        //                       rejects it. Your decorator silently
        //                       returns Ok(0) (EOF) instead of the
        //                       real byte count, breaking every reader
        //                       downstream.
        Ok(0)
        //^^^^^ *logical* bug: callers see EOF immediately.
    }
}

fn main() {}
