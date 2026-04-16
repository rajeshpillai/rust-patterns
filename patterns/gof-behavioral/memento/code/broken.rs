// Broken example — two Memento mistakes.
// This file is expected to FAIL to compile.
//
//   1. Store a reference to the state (instead of an owned clone)
//      in the history. Lifetimes collapse immediately — the history
//      can't outlive the state it points at.
//
//   2. Skip `#[derive(Clone)]` on the snapshot type. The snapshot()
//      method has no way to produce a copy.

pub struct EditorState {
    pub text: String,
    pub cursor: usize,
}
// No #[derive(Clone)].

pub struct Editor {
    state: EditorState,
}

// Mistake #1 — history of references.
pub struct BorrowedHistory<'a> {
    past: Vec<&'a EditorState>,
}

impl Editor {
    pub fn snapshot_into<'a>(&'a self, h: &mut BorrowedHistory<'a>) {
        h.past.push(&self.state);
        // Works in isolation — but now NOTHING can mutate
        // `self.state` while `h` exists (shared borrow outlives).
    }

    pub fn mutate(&mut self, _new: &str) {
        // Can't coexist with `snapshot_into` above: if the history
        // holds a `&EditorState`, the editor cannot take `&mut self`
        // without invalidating the borrow. E0502 in real use:
        //   error[E0502]: cannot borrow `*self` as mutable because
        //                 `self.state` is also borrowed as immutable
        self.state.text.push_str("x");
    }
}

// Mistake #2 — snapshot requires Clone but EditorState isn't.
pub fn snapshot(s: &EditorState) -> EditorState {
    s.clone()
    //^^^^^^ error[E0599]: no method named `clone` found for
    //       reference `&EditorState` — help: implement or derive
    //       `Clone`
}

fn main() {}
