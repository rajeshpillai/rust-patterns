// Idiomatic Rust — implement std::iter::Iterator once, get the whole
// library of combinators automatically.
//
// Two things to notice:
//   1. You implement a SINGLE method: `fn next(&mut self) -> Option<Item>`.
//      Everything else (map, filter, fold, collect, sum, product,
//      chain, take, enumerate, zip, all, any, count, find, ...) is
//      defined as a default method on the Iterator trait.
//
//   2. The adapters are LAZY. `iter.map(f).filter(pred)` builds a
//      structure that pulls one item through `f` and `pred` *per
//      item* when a consumer calls `.next()`. Nothing is allocated
//      until `.collect()` or similar is called.

// ---- A custom iterator: Fibonacci -------------------------------------

pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self { Self { curr: 0, next: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        // We never terminate — return None to stop. An infinite iterator
        // is fine in Rust because the consumers (take, take_while, ...)
        // are the ones that limit it.
        let out = self.curr;
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(out)
    }
}

// ---- A tree iterator over an in-order traversal ----------------------

pub struct Tree {
    pub value: i32,
    pub left:  Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

pub struct InOrder<'a> {
    stack: Vec<&'a Tree>,
    cur: Option<&'a Tree>,
}

impl<'a> Iterator for InOrder<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        while let Some(node) = self.cur {
            self.stack.push(node);
            self.cur = node.left.as_deref();
        }
        let node = self.stack.pop()?;
        self.cur = node.right.as_deref();
        Some(node.value)
    }
}

impl Tree {
    pub fn iter(&self) -> InOrder<'_> {
        InOrder { stack: Vec::new(), cur: Some(self) }
    }
}

// ---- Using the standard library combinators --------------------------

fn main() {
    // Get sum of squares of the first 10 Fibonacci numbers > 4.
    // No intermediate Vec is materialized — the chain is a pipeline.
    let total: u64 = Fibonacci::new()
        .take(10)
        .map(|n| n * n)
        .filter(|&n| n > 4)
        .sum();
    println!("Σ of squares of fib(0..10) that > 4 = {total}");

    // The first 5 primes via a simple sieve, expressed as combinators.
    let primes: Vec<u64> = (2u64..)
        .filter(|n| (2..=n / 2).all(|d| n % d != 0))
        .take(5)
        .collect();
    println!("first 5 primes = {primes:?}");

    // In-order traversal of a tree, collected into a Vec.
    let tree = Tree {
        value: 2,
        left:  Some(Box::new(Tree { value: 1, left: None, right: None })),
        right: Some(Box::new(Tree {
            value: 4,
            left:  Some(Box::new(Tree { value: 3, left: None, right: None })),
            right: Some(Box::new(Tree { value: 5, left: None, right: None })),
        })),
    };
    let in_order: Vec<i32> = tree.iter().collect();
    println!("in-order = {in_order:?}");

    // Zip + enumerate + take — the whole library composes.
    let zipped: Vec<_> = Fibonacci::new()
        .zip(['a', 'b', 'c', 'd'].iter())
        .enumerate()
        .take(4)
        .collect();
    println!("{zipped:?}");
}
