// Naive alternative — skip the newtype, pass raw primitives around.
//
// This compiles. It even runs. It is also the shape of bug that costs
// companies real money: `cancel_order(user_id)` silently does the
// wrong thing because both IDs are just `u64`.
//
// Kept here as the honest comparison. See code/idiomatic.rs for the
// newtype version that the compiler would have refused to compile.

fn find_user(_id: u64)   { /* ... */ }
fn cancel_order(_id: u64) { /* ... */ }

fn validate_email(s: &str) -> bool {
    s.contains('@') && s.contains('.')
}

// Every function that takes an email as &str has to remember to check.
// Miss one, and invalid data reaches the database / the SMTP server /
// the audit log.
fn send_welcome(to: &str) -> Result<(), &'static str> {
    if !validate_email(to) {
        return Err("invalid email");
    }
    println!("welcome email -> {to}");
    Ok(())
}

fn main() {
    let user_id: u64 = 42;
    let order_id: u64 = 7;

    // Silently wrong — the compiler has no way to know these shouldn't
    // be swapped. This is precisely the bug the Newtype pattern
    // eliminates.
    find_user(order_id);
    cancel_order(user_id);

    // Validation is a runtime thing. If you forget to call it, you
    // send welcome emails to non-email strings.
    let _ = send_welcome("rajesh@example.com");
    let _ = send_welcome("not-an-email"); // returns Err at runtime
}
