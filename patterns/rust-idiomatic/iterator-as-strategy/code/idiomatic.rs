// Iterator as Strategy — every iterator adapter is a Strategy pattern
// whose "strategy" is a closure you pass in.
//
// Five examples from different families:
//   * map              — Strategy: transform each item
//   * filter           — Strategy: predicate
//   * fold             — Strategy: accumulation step
//   * sort_by          — Strategy: comparator
//   * scan             — Strategy: stateful stepper
//
// Each closure is a one-line Strategy implementation that the
// standard library's adapter parameterizes over. No trait Strategy,
// no Box<dyn>, no ceremony.

use std::cmp::Ordering;

fn main() {
    let xs = vec![3_i64, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // 1. map — Strategy: "square each number"
    let squares: Vec<i64> = xs.iter().map(|&n| n * n).collect();
    println!("squares: {squares:?}");

    // 2. filter — Strategy: "keep only odd"
    let odd: Vec<i64> = xs.iter().copied().filter(|n| n % 2 == 1).collect();
    println!("odd: {odd:?}");

    // 3. fold — Strategy: "sum with bonus for even numbers"
    //    Demonstrates: the fold step function IS the algorithm.
    let weighted = xs.iter().fold(0_i64, |acc, &n| acc + if n % 2 == 0 { 2 * n } else { n });
    println!("weighted sum: {weighted}");

    // 4. sort_by — Strategy: "comparator". Sort by (parity, value):
    //    evens before odds, then ascending within each group.
    let mut ordered = xs.clone();
    ordered.sort_by(|a, b| match (a % 2, b % 2) {
        (0, 0) | (1, 1) => a.cmp(b),
        (0, _) => Ordering::Less,
        (_, _) => Ordering::Greater,
    });
    println!("sort_by parity then value: {ordered:?}");

    // 5. scan — Strategy: stateful stepper. Running cumulative sum
    //    that stops once it exceeds 20.
    let partials: Vec<i64> = xs.iter().scan(0_i64, |sum, &n| {
        *sum += n;
        if *sum > 20 { None } else { Some(*sum) }
    }).collect();
    println!("running sum until > 20: {partials:?}");

    // 6. Compose several strategies into a pipeline — no intermediate
    //    Vec; `take_while` stops the stream eagerly.
    let total: u64 = (0_u64..)
        .map(|n| n * n)
        .filter(|&sq| sq % 2 == 0)
        .take_while(|&sq| sq < 1000)
        .sum();
    println!("sum of even squares < 1000 = {total}");

    // 7. A generic helper that accepts any "reducer" strategy.
    //    Shows that YOUR API can take closures the same way std does.
    fn reduce<I, F, T>(iter: I, init: T, step: F) -> T
    where
        I: IntoIterator,
        F: FnMut(T, I::Item) -> T,
    {
        iter.into_iter().fold(init, step)
    }

    let joined = reduce(
        xs.iter().map(|n| n.to_string()),
        String::new(),
        |mut acc, s| { if !acc.is_empty() { acc.push(','); } acc.push_str(&s); acc },
    );
    println!("joined: {joined}");
}
