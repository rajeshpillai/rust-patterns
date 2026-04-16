// Broken example — the classic consuming-self footguns.
// This file is expected to FAIL to compile.
//
//   1. Save the builder, call .build() twice. First call moves the
//      builder; second call sees a moved value (E0382).
//
//   2. Try to call a setter on `&self` (borrowed) when it requires
//      `self` (owned). Common when someone assigns the builder to
//      an immutable binding and then tries to chain.

use std::time::Duration;

#[derive(Default)]
pub struct Builder {
    endpoint: Option<String>,
}

impl Builder {
    pub fn endpoint(mut self, e: impl Into<String>) -> Self {
        self.endpoint = Some(e.into());
        self
    }
    pub fn build(self) -> Option<String> { self.endpoint }
}

fn main() {
    let builder = Builder::default().endpoint("a");

    // Mistake #1 — call build twice on the same binding.
    let _c1 = builder.build();
    let _c2 = builder.build();
    //        ^^^^^^^ error[E0382]: use of moved value: `builder`

    // Mistake #2 — `let builder = ...` without `mut`, then try to
    // reassign mid-chain. Consuming-self setters don't touch the
    // binding; re-binding with `let` shadows.
    let b = Builder::default();
    b.endpoint("x");
    //^ this call moves `b` into the new Self, but we discarded the
    //  returned value. The next use of `b` would fail:
    let _ = b;
    //      ^ error[E0382]: use of moved value: `b`

    // The fix in both cases is to CHAIN the calls or RE-BIND with
    // `let b = b.endpoint("x");` (note: no mut needed — shadowing
    // is how consuming-self builders work).

    let _ = Duration::from_secs(1); // keep Duration import used
}
