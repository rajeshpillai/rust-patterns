// Broken example — try to use a FileHandle after explicitly dropping
// it. The borrow checker refuses: `drop(h)` is a move into the
// function `std::mem::drop`, after which `h` is no longer usable.
//
// This file is expected to FAIL to compile with E0382.

pub struct FileHandle {
    fd: i32,
}

impl FileHandle {
    pub fn new(fd: i32) -> Self { Self { fd } }
    pub fn write(&mut self, _bytes: &[u8]) { /* ... */ }
}

impl Drop for FileHandle {
    fn drop(&mut self) { println!("[close] fd={}", self.fd); }
}

fn main() {
    let mut h = FileHandle::new(3);
    h.write(b"first");

    // `drop` is a normal function with signature `fn drop<T>(_: T)`.
    // Calling it moves `h` into `drop`, which consumes it. After this
    // line, `h` is uninitialized — the borrow checker tracks that.
    drop(h);

    h.write(b"second");
    //^ error[E0382]: borrow of moved value: `h`
    //  value used here after move
}
