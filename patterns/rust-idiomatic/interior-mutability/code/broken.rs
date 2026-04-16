// Broken example — the two most common interior-mutability traps.
// This file is expected to FAIL to compile OR panic at runtime; the
// comments note which case each mistake produces.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

// Mistake #1 — try to share a RefCell across threads.
// RefCell is Send but NOT Sync. Thread::spawn requires the captured
// value to be Send + 'static; Arc<RefCell<T>> is not Sync, so another
// thread cannot borrow the RefCell at all. Compiler stops you:
pub fn cross_thread() {
    let data = std::sync::Arc::new(RefCell::new(0_u64));
    let d2 = data.clone();

    std::thread::spawn(move || {
        *d2.borrow_mut() += 1;
        //  ^^^^^^^^^^^^ error[E0277]: `RefCell<u64>` cannot be shared
        //               between threads safely
        //               within `Arc<RefCell<u64>>`, the trait `Sync`
        //               is not implemented for `RefCell<u64>`
    });
    // The fix is Mutex<u64> (or an atomic), not RefCell.
}

// Mistake #2 — double borrow at runtime.
// RefCell enforces aliasing XOR mutation *at runtime*; if you ask for
// a mutable borrow while another borrow is live, it panics. This
// compiles; it fails at runtime.
pub fn double_borrow() {
    let c = RefCell::new(vec![1_u32, 2, 3]);
    let read = c.borrow();          // & view into the Vec
    let _write = c.borrow_mut();    // RUNTIME PANIC: already borrowed
    println!("{read:?}");
    // thread 'main' panicked at 'already borrowed: BorrowMutError'
}

// Mistake #3 — holding a MutexGuard across an .await or across a
// long operation. This deadlocks or causes a scheduler stall. We
// can't show the async .await case without tokio, but the shape is:
//
//   let g = mutex.lock().await;        // guard held...
//   some_long_thing().await;           // ...across an await point
//   // now another task on this thread is blocked on this lock
//
// For the sync version, the problem is holding the guard longer
// than necessary while performing slow I/O — during which time no
// other thread can lock.
pub fn long_held_guard(m: &Mutex<Vec<u32>>) {
    let mut g = m.lock().expect("mutex poisoned");
    g.push(1);
    // pretend this is a slow operation
    std::thread::sleep(std::time::Duration::from_millis(1000));
    g.push(2);
    // every other thread trying to m.lock() blocks for the whole second.
}

fn main() {
    cross_thread();
    double_borrow();
    long_held_guard(&Mutex::new(vec![]));
}
