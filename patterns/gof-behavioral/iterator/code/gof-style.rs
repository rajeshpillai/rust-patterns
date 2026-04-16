// Direct GoF translation — a separate `Iterator` trait object with
// hasNext / next methods, a Collection trait with `iterator()`. This
// is what you'd write in Rust if you squinted at the Java port of
// the pattern and translated word by word.
//
// It works. It is also strictly worse than just implementing
// std::iter::Iterator (see code/idiomatic.rs) because you lose the
// fifty combinators the standard library gives you for free.

pub trait GofIterator {
    type Item;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Collection {
    type Item;
    type Iter: GofIterator<Item = Self::Item>;
    fn iterator(&self) -> Self::Iter;
}

// A simple owned collection.
pub struct Bag {
    items: Vec<i32>,
}

pub struct BagIter<'a> {
    items: &'a [i32],
    idx: usize,
}

impl GofIterator for BagIter<'_> {
    type Item = i32;
    fn has_next(&self) -> bool { self.idx < self.items.len() }
    fn next(&mut self) -> Option<i32> {
        if self.has_next() {
            let v = self.items[self.idx];
            self.idx += 1;
            Some(v)
        } else {
            None
        }
    }
}

impl Collection for Bag {
    type Item = i32;
    type Iter = BagIter<'static>;
    fn iterator(&self) -> Self::Iter {
        // This is awkward in Rust: a GoF-style iterator over `&self`
        // wants a lifetime it doesn't have from `Self::Iter`. A real
        // Rust iterator would use GATs or just implement std's
        // Iterator. This gof-style struct papers over the issue by
        // cloning into a Vec — which is the kind of tax the classical
        // pattern keeps charging.
        let leaked: &'static [i32] = Box::leak(self.items.clone().into_boxed_slice());
        BagIter { items: leaked, idx: 0 }
    }
}

fn main() {
    let bag = Bag { items: vec![1, 2, 3, 4, 5] };
    let mut it = bag.iterator();

    // Every operation is hand-rolled. Want the sum? Write a while
    // loop. Want squares? Write another loop. Want filter +
    // fold? Write a third loop. None of the standard library's
    // combinators help you.
    let mut sum = 0;
    while it.has_next() {
        if let Some(v) = it.next() {
            sum += v;
        }
    }
    println!("sum = {sum}");
}
