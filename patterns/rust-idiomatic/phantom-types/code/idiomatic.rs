// PhantomData — how to use a generic parameter you don't actually
// store. Three canonical uses in one file:
//
//   1. Units of measure — Wrapping<Seconds> vs Wrapping<Milliseconds>.
//      Same bytes, distinct types, mixing them is a compile error.
//
//   2. Identifier disambiguation — Id<User> vs Id<Order>. Same shape
//      (u64), different meaning, tracked by the type system.
//
//   3. Variance + auto-trait control — PhantomData<fn() -> T> makes
//      a handle covariant in T but stops auto-Send/Sync. Demonstrated
//      in the `RawHandle` definition at the bottom.

use std::marker::PhantomData;

// ---- 1. Units of measure ---------------------------------------------

pub struct Seconds;
pub struct Milliseconds;

// PhantomData is required — without it, `Unit` is an "unused generic
// parameter" and the compiler rejects the definition (E0392).
pub struct Duration<Unit> {
    amount: u64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Duration<Unit> {
    pub const fn new(amount: u64) -> Self {
        Self { amount, _unit: PhantomData }
    }
    pub const fn amount(&self) -> u64 { self.amount }
}

// Only available when Unit is *specifically* Seconds.
impl Duration<Seconds> {
    pub fn to_ms(self) -> Duration<Milliseconds> {
        Duration::new(self.amount * 1000)
    }
}

// Only available when Unit is *specifically* Milliseconds.
impl Duration<Milliseconds> {
    pub fn to_secs(self) -> Duration<Seconds> {
        Duration::new(self.amount / 1000)
    }
}

// ---- 2. Typed identifiers --------------------------------------------

pub struct User;
pub struct Order;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id<T> {
    value: u64,
    _type: PhantomData<fn() -> T>,   // see note at #3 below
}

impl<T> Id<T> {
    pub const fn new(value: u64) -> Self {
        Self { value, _type: PhantomData }
    }
    pub const fn raw(&self) -> u64 { self.value }
}

// Domain functions. Mixing these up is a compile error, not a
// production incident. See also the Newtype pattern for the
// non-generic version of this idea.
pub fn find_user(_id: Id<User>) { /* ... */ }
pub fn cancel_order(_id: Id<Order>) { /* ... */ }

// ---- 3. Variance / auto-trait control --------------------------------

// A `RawHandle<T>` acts as a typed wrapper over a raw OS handle. We
// want it to:
//   * be covariant in T — subtyping should pass through
//   * NOT be automatically Send/Sync (the underlying handle isn't
//     safe to share without synchronization)
//
// `PhantomData<fn() -> T>` achieves both: `fn() -> T` is covariant in
// T, and function-pointer PhantomData is `!Send + !Sync` by default.
pub struct RawHandle<T> {
    fd: i32,
    _t: PhantomData<fn() -> T>,
}

// If you wanted the opposite — "hold a borrowed reference's lifetime" —
// you'd write `PhantomData<&'a T>`. If you wanted "owns a T in spirit
// but doesn't store it" (affecting Drop-check), `PhantomData<T>`. The
// table is in the std::marker::PhantomData rustdoc.

fn main() {
    // Units of measure
    let t: Duration<Seconds> = Duration::new(5);
    let t_ms: Duration<Milliseconds> = t.to_ms();
    println!("5 s = {} ms", t_ms.amount());

    // Uncomment to see E0308 on unit mismatch:
    //   let bad = Duration::<Seconds>::new(5).amount() + Duration::<Milliseconds>::new(5).amount();
    //   The amounts are both u64 so that DOES compile as raw arithmetic, but anywhere a
    //   `Duration<Seconds>` is expected, passing `Duration<Milliseconds>` is rejected.

    // Typed identifiers
    let u: Id<User> = Id::new(42);
    let o: Id<Order> = Id::new(7);
    find_user(u);
    cancel_order(o);
    // cancel_order(u);   // <- E0308 if uncommented
    println!("ids: u={}, o={}", u.raw(), o.raw());

    let _h: RawHandle<u8> = RawHandle { fd: 3, _t: PhantomData };
}
