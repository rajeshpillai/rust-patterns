// Naive alternative — manual close(), no Drop impl. This is what the
// code looks like in languages without RAII.
//
// The bug model this invites:
//   * Forget to call .close() on a path that returns early via `?`.
//   * Forget to call .close() in an error branch.
//   * Call .close() twice.
//   * Call other methods after .close().
//
// Rust does not forbid this style, but Drop makes it unnecessary and
// unsafe-feeling. Kept here for contrast only.

#[derive(Debug)]
pub struct FileHandle {
    fd: Option<i32>,
}

impl FileHandle {
    pub fn open(path: &str) -> Result<Self, &'static str> {
        if path.is_empty() { return Err("empty path"); }
        println!("[open]  {path}");
        Ok(Self { fd: Some(0x42) })
    }
    pub fn write(&mut self, _bytes: &[u8]) -> Result<(), &'static str> {
        self.fd.ok_or("closed")?;
        println!("[write] ok");
        Ok(())
    }
    pub fn close(&mut self) {
        if let Some(fd) = self.fd.take() {
            println!("[close] fd={fd:#x}");
        }
    }
}

// Notice how the manual-close discipline infects every call site.
// Forget one early-return and you leak the handle.
fn process(path: &str) -> Result<(), &'static str> {
    let mut h = FileHandle::open(path)?;
    h.write(b"hello")?;
    h.close();   // must remember on success
    Ok(())
}

fn forgotten_on_error(path: &str) -> Result<(), &'static str> {
    let mut h = FileHandle::open(path)?;
    h.write(b"hello")?;  // if this returns Err, we skip the close() below — leak!
    h.close();
    Ok(())
}

fn main() {
    let _ = process("happy.txt");
    let _ = forgotten_on_error("partial.txt");
    // ^ if write() had failed, the handle would have leaked.
    //   With Drop (see code/idiomatic.rs) this is impossible.
}
