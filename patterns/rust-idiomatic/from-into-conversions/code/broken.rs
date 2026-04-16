// Broken example — the two most common From/Into mistakes.
// This file is expected to FAIL to compile.
//
//   1. Impl `Into<T>` instead of `From<T>`. The blanket impl
//      `impl<T, U: From<T>> Into<T> for U` gives you Into for free
//      whenever you impl From. Writing a manual `impl Into` conflicts
//      with the blanket and is usually wrong.
//
//   2. Use `From` for a conversion that can fail. The `From::from`
//      signature returns `Self` (not `Result<Self, _>`), so the body
//      either lies (silent failure), panics, or refuses to compile
//      if you try to return a Result.

// Mistake #1 — manual Into impl conflicts with the blanket impl.
pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Fahrenheit { Fahrenheit(c.0 * 9.0 / 5.0 + 32.0) }
}

// This clashes with the blanket `impl Into<Fahrenheit> for Celsius`
// that the `From` impl above already provides:
impl Into<Fahrenheit> for Celsius {
    //^^^^^^^^^^^^^^^^^^^^^^^^^ error[E0119]: conflicting implementations
    //                          of trait `Into<Fahrenheit>` for type `Celsius`
    fn into(self) -> Fahrenheit { Fahrenheit(self.0 * 9.0 / 5.0 + 32.0) }
}

// Mistake #2 — use From for a fallible conversion.
pub struct Port(u16);

// Signature of From::from is `fn from(U) -> Self`. No Result available.
impl From<i64> for Port {
    fn from(v: i64) -> Self {
        // Two bad choices:
        //   (a) panic on out-of-range (hides the error, poor library design)
        //   (b) return Self with garbage data (silent corruption)
        Port(v as u16)
        // The right fix is TryFrom<i64> for Port — see code/idiomatic.rs.
    }
}

fn main() {}
