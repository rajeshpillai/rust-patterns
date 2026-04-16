// Idiomatic Rust — two shapes, pick by what you need:
//
//   A) Enum + match — a closed set of commands that you can store,
//      inspect, serialize, log, and undo. The only downside is you
//      can't add new commands from outside this module without
//      adding a variant — which is usually a feature, not a limit.
//
//   B) Closure — Box<dyn FnOnce()> for "just defer this work". One
//      line of code, zero boilerplate; you lose introspection,
//      serialization, and undo.

// ---- A) Enum-based commands with undo --------------------------------

#[derive(Debug)]
pub struct Editor {
    pub lines: Vec<String>,
}

// Commands hold both the forward data (what to do) and any state
// captured during execute() so they can undo themselves.
#[derive(Debug)]
pub enum Cmd {
    AddLine { text: String, inserted_at: Option<usize> },
    DeleteLine { index: usize, removed: Option<String> },
}

impl Cmd {
    pub fn add_line(text: impl Into<String>) -> Self {
        Cmd::AddLine { text: text.into(), inserted_at: None }
    }
    pub fn delete_line(index: usize) -> Self {
        Cmd::DeleteLine { index, removed: None }
    }

    pub fn execute(&mut self, doc: &mut Editor) {
        match self {
            Cmd::AddLine { text, inserted_at } => {
                doc.lines.push(text.clone());
                *inserted_at = Some(doc.lines.len() - 1);
            }
            Cmd::DeleteLine { index, removed } => {
                if *index < doc.lines.len() {
                    *removed = Some(doc.lines.remove(*index));
                }
            }
        }
    }

    pub fn undo(&mut self, doc: &mut Editor) {
        match self {
            Cmd::AddLine { inserted_at, .. } => {
                if let Some(i) = inserted_at.take() { doc.lines.remove(i); }
            }
            Cmd::DeleteLine { index, removed } => {
                if let Some(line) = removed.take() { doc.lines.insert(*index, line); }
            }
        }
    }
}

#[derive(Default)]
pub struct Invoker {
    history: Vec<Cmd>,
    redo:    Vec<Cmd>,
}

impl Invoker {
    pub fn submit(&mut self, mut cmd: Cmd, doc: &mut Editor) {
        cmd.execute(doc);
        self.history.push(cmd);
        self.redo.clear();
    }
    pub fn undo(&mut self, doc: &mut Editor) {
        if let Some(mut cmd) = self.history.pop() {
            cmd.undo(doc);
            self.redo.push(cmd);
        }
    }
    pub fn redo(&mut self, doc: &mut Editor) {
        if let Some(mut cmd) = self.redo.pop() {
            cmd.execute(doc);
            self.history.push(cmd);
        }
    }
}

// ---- B) Closure queue — deferred work with no undo -------------------

use std::collections::VecDeque;

pub struct Queue {
    work: VecDeque<Box<dyn FnOnce() + Send + 'static>>,
}

impl Queue {
    pub fn new() -> Self { Self { work: VecDeque::new() } }

    // FnOnce — each command is called exactly once and consumed.
    // The `Send + 'static` bounds make the queue safe to hand to a
    // worker thread; drop them if single-threaded.
    pub fn submit<F: FnOnce() + Send + 'static>(&mut self, f: F) {
        self.work.push_back(Box::new(f));
    }
    pub fn run_all(&mut self) {
        while let Some(cmd) = self.work.pop_front() {
            cmd();
        }
    }
}
impl Default for Queue { fn default() -> Self { Self::new() } }

fn main() {
    // A) Enum with undo/redo
    let mut doc = Editor { lines: vec![] };
    let mut inv = Invoker::default();

    inv.submit(Cmd::add_line("hello"), &mut doc);
    inv.submit(Cmd::add_line("world"), &mut doc);
    println!("{:?}", doc.lines);
    inv.undo(&mut doc);
    println!("undone: {:?}", doc.lines);
    inv.redo(&mut doc);
    println!("redone: {:?}", doc.lines);

    // B) Deferred closure commands
    let mut queue = Queue::new();
    queue.submit(|| println!("[queued] hello"));
    queue.submit(move || println!("[queued] world"));
    queue.run_all();
}
