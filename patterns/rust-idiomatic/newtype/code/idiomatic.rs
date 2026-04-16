// Newtype — wrap a primitive in a tuple struct so the type system
// carries meaning the primitive alone cannot.
//
// Two uses covered here:
//   1) Distinct types for distinct meanings.
//      UserId and OrderId both wrap u64, but mixing them is a
//      compile error, not a production incident.
//
//   2) "Parse, don't validate."
//      EmailAddress can only be constructed via `parse`. Once you
//      hold an EmailAddress value, the type is a proof that the
//      string inside is a valid email. Functions that accept
//      `EmailAddress` never need to re-check.

use std::fmt;

// ---- Distinct identity ---------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OrderId(u64);

impl UserId  { pub fn new(n: u64) -> Self { Self(n) } pub fn as_u64(self) -> u64 { self.0 } }
impl OrderId { pub fn new(n: u64) -> Self { Self(n) } pub fn as_u64(self) -> u64 { self.0 } }

fn find_user(_u: UserId)   { /* ... */ }
fn cancel_order(_o: OrderId) { /* ... */ }

// ---- Parse, don't validate ----------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(String);

#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidEmail {
    Empty,
    NoAtSign,
    NoDomain,
}

impl fmt::Display for InvalidEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty     => f.write_str("email is empty"),
            Self::NoAtSign  => f.write_str("email is missing '@'"),
            Self::NoDomain  => f.write_str("email has no domain"),
        }
    }
}
impl std::error::Error for InvalidEmail {}

impl EmailAddress {
    // The ONLY way to construct an EmailAddress. Because `EmailAddress`
    // has a private inner field, external code cannot bypass this check.
    pub fn parse(raw: impl Into<String>) -> Result<Self, InvalidEmail> {
        let s = raw.into();
        if s.is_empty() { return Err(InvalidEmail::Empty); }
        let (local, domain) = s.split_once('@').ok_or(InvalidEmail::NoAtSign)?;
        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Err(InvalidEmail::NoDomain);
        }
        Ok(EmailAddress(s))
    }

    // A cheap borrowed accessor. Never expose the inner String mutably —
    // that would allow a caller to smuggle invalid state back in.
    pub fn as_str(&self) -> &str { &self.0 }
}

// This function's signature is a proof: whoever called it already held
// a validated email. We do no re-checking, and we can't be called with
// a plain `&str` by mistake.
fn send_welcome(to: &EmailAddress) {
    println!("welcome email -> {}", to.as_str());
}

fn main() -> Result<(), InvalidEmail> {
    // Distinct identity — mixing IDs is a compile error. Uncomment
    // either of these to see the error:
    //   cancel_order(UserId::new(42));         // E0308
    //   find_user(OrderId::new(42));           // E0308
    find_user(UserId::new(42));
    cancel_order(OrderId::new(7));

    // Parse at the boundary. Validation happens exactly once.
    let email = EmailAddress::parse("rajesh@example.com")?;
    send_welcome(&email);

    // Invalid input surfaces as a typed error, not a panic.
    let err = EmailAddress::parse("not-an-email").unwrap_err();
    println!("parse error (expected): {err}");
    Ok(())
}
