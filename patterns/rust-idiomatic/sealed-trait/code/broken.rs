// Broken example — a downstream-style impl trying to satisfy a sealed
// public trait. The seal is provided by a supertrait the downstream
// cannot name, so the impl fails with E0277 (the supertrait bound is
// not satisfied).
//
// This file is expected to FAIL to compile.

// Simulate the upstream crate: public trait + private supertrait.
mod upstream {
    // In real code, this `private` module would be private at the
    // crate root. For a single-file demo, we expose it so the
    // downstream code below is representative, but the downstream
    // part is still not allowed to construct a Sealed impl because
    // the *path* to Sealed is intentionally not re-exported.
    mod private {
        pub trait Sealed {}
    }

    pub trait Format: private::Sealed {
        fn name(&self) -> &'static str;
    }

    pub struct Json;
    impl private::Sealed for Json {}
    impl Format for Json {
        fn name(&self) -> &'static str { "json" }
    }
}

// Simulate a downstream crate — it can USE `Format` as a bound but
// cannot IMPLEMENT it for its own type, because the Sealed supertrait
// is in `upstream::private`, which downstream code can't path to.
struct MyFormat;

impl upstream::Format for MyFormat {
    //  ^^^^^^^^^^^^^^^ error[E0277]: the trait bound
    //                  `MyFormat: upstream::private::Sealed`
    //                  is not satisfied
    fn name(&self) -> &'static str { "mine" }
}

fn main() {}
