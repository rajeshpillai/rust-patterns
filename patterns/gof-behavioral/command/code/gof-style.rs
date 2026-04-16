// Classical GoF Command — trait + Box<dyn Command> with execute/undo,
// owned by an Invoker that keeps undo/redo stacks.
//
// This is the right shape when commands come from plugins / user code
// and you can't close the set. For a closed set, see the enum form in
// code/idiomatic.rs. For a "just defer this work" case, see the
// Box<dyn FnOnce()> form there too.

pub struct Editor {
    pub lines: Vec<String>,
}

pub trait Command {
    fn execute(&mut self, doc: &mut Editor);
    fn undo(&mut self, doc: &mut Editor);
    fn name(&self) -> &'static str;
}

// ---- Concrete commands ------------------------------------------------

pub struct AddLine {
    text: String,
    // where it was added, so undo knows what to pop.
    inserted_at: Option<usize>,
}
impl AddLine {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), inserted_at: None }
    }
}
impl Command for AddLine {
    fn execute(&mut self, doc: &mut Editor) {
        doc.lines.push(self.text.clone());
        self.inserted_at = Some(doc.lines.len() - 1);
    }
    fn undo(&mut self, doc: &mut Editor) {
        if let Some(i) = self.inserted_at.take() {
            doc.lines.remove(i);
        }
    }
    fn name(&self) -> &'static str { "add-line" }
}

pub struct DeleteLine {
    index: usize,
    removed: Option<String>,
}
impl DeleteLine {
    pub fn at(index: usize) -> Self { Self { index, removed: None } }
}
impl Command for DeleteLine {
    fn execute(&mut self, doc: &mut Editor) {
        if self.index < doc.lines.len() {
            self.removed = Some(doc.lines.remove(self.index));
        }
    }
    fn undo(&mut self, doc: &mut Editor) {
        if let Some(line) = self.removed.take() {
            doc.lines.insert(self.index, line);
        }
    }
    fn name(&self) -> &'static str { "delete-line" }
}

// ---- Invoker ---------------------------------------------------------

#[derive(Default)]
pub struct Invoker {
    history: Vec<Box<dyn Command>>,
    redo:    Vec<Box<dyn Command>>,
}

impl Invoker {
    pub fn submit(&mut self, mut cmd: Box<dyn Command>, doc: &mut Editor) {
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

fn main() {
    let mut doc = Editor { lines: vec![] };
    let mut inv = Invoker::default();

    inv.submit(Box::new(AddLine::new("hello")), &mut doc);
    inv.submit(Box::new(AddLine::new("world")), &mut doc);
    println!("{:?}", doc.lines);       // [hello, world]

    inv.undo(&mut doc);
    println!("{:?}", doc.lines);       // [hello]

    inv.submit(Box::new(DeleteLine::at(0)), &mut doc);
    println!("{:?}", doc.lines);       // []

    inv.undo(&mut doc);
    println!("{:?}", doc.lines);       // [hello]
}
