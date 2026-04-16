// Memento — capture a snapshot so the state can be restored later.
// In Rust, "snapshot" is `#[derive(Clone)]` on the relevant fields.
// The pattern's remaining design: who holds the history, how deep
// does it go, what clears the future stack?

#[derive(Clone, Debug)]
pub struct EditorState {
    pub text: String,
    pub cursor: usize,
}

pub struct Editor {
    state: EditorState,
    history: History,
}

#[derive(Default)]
pub struct History {
    past: Vec<EditorState>,
    future: Vec<EditorState>,
    // Cap to avoid unbounded memory growth.
    max: usize,
}

impl History {
    pub fn new(max: usize) -> Self { Self { past: Vec::new(), future: Vec::new(), max } }

    pub fn save(&mut self, s: EditorState) {
        self.past.push(s);
        // Any new save invalidates the redo stack — standard editor behavior.
        self.future.clear();
        if self.past.len() > self.max {
            self.past.remove(0);
        }
    }

    pub fn undo(&mut self, current: EditorState) -> Option<EditorState> {
        let previous = self.past.pop()?;
        self.future.push(current);
        Some(previous)
    }

    pub fn redo(&mut self, current: EditorState) -> Option<EditorState> {
        let next = self.future.pop()?;
        self.past.push(current);
        Some(next)
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            state: EditorState { text: String::new(), cursor: 0 },
            history: History::new(100),
        }
    }

    pub fn text(&self) -> &str { &self.state.text }
    pub fn cursor(&self) -> usize { self.state.cursor }

    /// Every mutation snapshots the BEFORE state.
    fn checkpoint(&mut self) {
        self.history.save(self.state.clone());
    }

    pub fn insert(&mut self, s: &str) {
        self.checkpoint();
        self.state.text.insert_str(self.state.cursor, s);
        self.state.cursor += s.len();
    }

    pub fn backspace(&mut self) {
        if self.state.cursor == 0 { return; }
        self.checkpoint();
        // Handle the simple ASCII case only for brevity.
        self.state.text.remove(self.state.cursor - 1);
        self.state.cursor -= 1;
    }

    pub fn undo(&mut self) -> bool {
        match self.history.undo(self.state.clone()) {
            Some(prev) => { self.state = prev; true }
            None => false,
        }
    }

    pub fn redo(&mut self) -> bool {
        match self.history.redo(self.state.clone()) {
            Some(next) => { self.state = next; true }
            None => false,
        }
    }
}

impl Default for Editor { fn default() -> Self { Self::new() } }

fn main() {
    let mut ed = Editor::new();
    ed.insert("hello ");
    ed.insert("world");
    println!("after inserts:   {:?}", ed.text());

    ed.undo(); println!("after 1 undo:    {:?}", ed.text());
    ed.undo(); println!("after 2 undos:   {:?}", ed.text());
    ed.redo(); println!("after redo:      {:?}", ed.text());

    // New action after undo clears the future.
    ed.insert(" pattern");
    println!("after new edit:  {:?}", ed.text());
    println!("redo available?: {}", ed.redo());   // false — cleared
}
