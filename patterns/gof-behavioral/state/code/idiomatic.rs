// Idiomatic Rust — state is an enum on the Post itself. Transitions
// consume `self` and return a new Post, so every illegal transition
// is a `Result::Err`, not a silent no-op.
//
// This is the "runtime-checked" form — see the Typestate pattern for
// the compile-time upgrade where illegal transitions do not compile.

#[derive(Debug)]
pub enum Status {
    Draft,
    Pending,
    Published,
}

#[derive(Debug)]
pub struct Post {
    pub body: String,
    pub status: Status,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TransitionError {
    CannotSubmit  { from: &'static str },
    CannotApprove { from: &'static str },
    CannotReject  { from: &'static str },
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionError::CannotSubmit  { from } => write!(f, "cannot submit from {from}"),
            TransitionError::CannotApprove { from } => write!(f, "cannot approve from {from}"),
            TransitionError::CannotReject  { from } => write!(f, "cannot reject from {from}"),
        }
    }
}
impl std::error::Error for TransitionError {}

impl Post {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into(), status: Status::Draft }
    }

    pub fn submit(self) -> Result<Self, TransitionError> {
        match self.status {
            Status::Draft => Ok(Self { status: Status::Pending, ..self }),
            Status::Pending   => Err(TransitionError::CannotSubmit { from: "Pending" }),
            Status::Published => Err(TransitionError::CannotSubmit { from: "Published" }),
        }
    }

    pub fn approve(self) -> Result<Self, TransitionError> {
        match self.status {
            Status::Pending => Ok(Self { status: Status::Published, ..self }),
            Status::Draft     => Err(TransitionError::CannotApprove { from: "Draft" }),
            Status::Published => Err(TransitionError::CannotApprove { from: "Published" }),
        }
    }

    pub fn reject(self) -> Result<Self, TransitionError> {
        match self.status {
            Status::Pending => Ok(Self { status: Status::Draft, ..self }),
            Status::Draft     => Err(TransitionError::CannotReject { from: "Draft" }),
            Status::Published => Err(TransitionError::CannotReject { from: "Published" }),
        }
    }
}

fn main() -> Result<(), TransitionError> {
    // Happy path — the compiler does not stop us from writing this in a
    // wrong order, but the runtime check will.
    let post = Post::new("Hello, world")
        .submit()?
        .approve()?;
    println!("{:?} — {}", post.status, post.body);

    // Illegal transition is a typed Err, not a silent no-op like in the
    // GoF translation. See code/broken.rs for how to misuse this API,
    // and rust-idiomatic/typestate for the compile-time version.
    let err = Post::new("oops").approve().unwrap_err();
    println!("expected error: {err}");
    Ok(())
}
