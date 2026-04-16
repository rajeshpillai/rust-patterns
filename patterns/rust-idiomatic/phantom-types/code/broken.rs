// Broken example — what happens if you forget PhantomData.
//
// A struct with a type parameter that isn't used by any field is
// rejected with E0392: "parameter `Unit` is never used". The compiler
// insists that every type parameter either be stored or be declared
// as phantom.
//
// This file is expected to FAIL to compile.

// Mistake #1: generic parameter `Unit` not referenced anywhere.
pub struct Duration<Unit> {
    amount: u64,
}
// ^^^^^^^^^^^^^^^^^^^^^^ error[E0392]: parameter `Unit` is never used
//                        consider removing `Unit`, referring to it in a
//                        field, or using a marker such as `PhantomData`

// Mistake #2: pass a Duration<Seconds> where Duration<Milliseconds>
// is expected. Without PhantomData, the above doesn't even compile;
// with PhantomData (see code/idiomatic.rs), this is the downstream
// error you'd see.
pub struct Seconds;
pub struct Milliseconds;

pub fn sleep_for_ms(_d: Duration<Milliseconds>) { /* ... */ }

fn main() {
    // If we fixed #1, this call would still be wrong:
    //   sleep_for_ms(Duration::<Seconds> { amount: 5 });
    //   ^^^^^^^^^^^^ error[E0308]: mismatched types
    //                expected `Duration<Milliseconds>`,
    //                found `Duration<Seconds>`
}
