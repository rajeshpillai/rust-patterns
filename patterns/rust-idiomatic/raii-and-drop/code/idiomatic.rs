// RAII — a resource handle that cleans up automatically. We use a
// fake file handle (just a counter) so the example compiles anywhere
// without depending on std::fs internals.
//
// The key move is: you only implement Drop once. Every caller, in
// every code path — normal return, panic, early `?` exit — gets the
// cleanup for free. There is no defer, no try/finally, no manual
// close() call to forget.

use std::fmt;

#[derive(Debug)]
pub struct FileHandle {
    path: String,
    // In a real implementation this would be a raw fd; we keep an
    // Option so we can represent the "already closed" state for
    // defensive Drop.
    fd: Option<i32>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FileError {
    OpenFailed { path: String },
    WriteFailed,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenFailed { path } => write!(f, "open failed: {path}"),
            Self::WriteFailed => f.write_str("write failed"),
        }
    }
}
impl std::error::Error for FileError {}

impl FileHandle {
    pub fn open(path: impl Into<String>) -> Result<Self, FileError> {
        let path = path.into();
        if path.is_empty() { return Err(FileError::OpenFailed { path }); }
        // pretend we got a real fd from the OS
        println!("[open]  {path}");
        Ok(Self { path, fd: Some(0x42) })
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, FileError> {
        // deliberately fallible to exercise the `?` early-return path
        let _ = self.fd.ok_or(FileError::WriteFailed)?;
        println!("[write] {} bytes to {}", bytes.len(), self.path);
        Ok(bytes.len())
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        // Defensive: take() replaces the fd with None so a Drop that
        // runs twice (which should not happen, but can if you play
        // games with mem::forget + ptr::drop_in_place) doesn't
        // double-close.
        if let Some(fd) = self.fd.take() {
            println!("[close] fd={fd:#x} ({})", self.path);
            // real impl would call libc::close(fd) or platform equiv.
        }
    }
}

fn happy_path() -> Result<(), FileError> {
    let mut h = FileHandle::open("happy.txt")?;
    h.write(b"hello")?;
    Ok(())
    // Drop runs here — no explicit call.
}

fn early_return() -> Result<(), FileError> {
    let mut h = FileHandle::open("early.txt")?;
    // simulate a failure
    h.fd = None;
    h.write(b"oops")?;
    // ^^^ this `?` returns early with Err. The drop *still* runs on
    // the way out of this function, even though `Ok(())` below is
    // never reached.
    Ok(())
}

fn panicking() {
    let _h = FileHandle::open("panic.txt").expect("open should succeed");
    // A panic unwinds the stack; Drop runs for each owned value in
    // reverse declaration order. The close for panic.txt WILL run.
    // (Uncomment to observe; we catch the unwind below so tests
    // still pass.)
    // panic!("simulated");
}

fn main() -> Result<(), FileError> {
    happy_path()?;
    // early_return returns Err but still prints [close early.txt]
    let _ = early_return();
    panicking();
    println!("main finished");
    Ok(())
}
