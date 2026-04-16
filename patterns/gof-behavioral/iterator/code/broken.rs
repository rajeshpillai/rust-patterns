// Broken example — two classic "Rust iterators aren't like Java's"
// failures. Both compile if you fix them; this file shows them as
// deliberate errors for teaching.
//
// This file is expected to FAIL to compile.

// Mistake #1: calling `.next()` on an iterator you don't have mutable
// access to. `Iterator::next` is `fn next(&mut self)` — iteration
// advances state, so you must own the iterator mutably.
pub fn first_element(xs: Vec<i32>) -> Option<i32> {
    // `xs.iter()` returns Iter<'_, i32> — immutable reference to self.
    // Assigning to a `let` binding makes it movable but not mutable.
    let it = xs.iter();
    it.next()
    //^^^^^^^ error[E0596]: cannot borrow `it` as mutable, as it is not
    //        declared as mutable
}

// Mistake #2: use the iterator after collecting. Once `.collect()`
// (or any consumer) has run, the iterator is consumed — it's gone,
// dropped, moved, exhausted. Reaching for it is E0382.
pub fn count_and_collect(xs: Vec<i32>) -> (Vec<i32>, usize) {
    let it = xs.into_iter();
    let collected: Vec<i32> = it.collect();
    let counted = it.count();
    //            ^^ error[E0382]: use of moved value: `it`
    (collected, counted)
}

fn main() {
    let _ = first_element(vec![1, 2, 3]);
    let _ = count_and_collect(vec![1, 2, 3]);
}
