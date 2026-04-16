// Broken example — accidentally mixing two newtypes that share the
// same underlying shape. The whole point of Newtype is that this is
// rejected by the compiler, so this file is expected to FAIL with E0308.

#[derive(Clone, Copy, Debug)]
pub struct UserId(u64);

#[derive(Clone, Copy, Debug)]
pub struct OrderId(u64);

fn cancel_order(_o: OrderId) {
    // imagine this is DELETE FROM orders WHERE id = ?
}

fn main() {
    let user = UserId(42);

    // A plain `u64` would compile and silently delete the wrong row.
    // With Newtype the compiler refuses: you cannot pass a UserId
    // where an OrderId is expected.
    cancel_order(user);
    //           ^^^^ error[E0308]: mismatched types
    //                expected struct `OrderId`, found struct `UserId`
}
