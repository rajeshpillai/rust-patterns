// Composite — recursive enum in Rust. A `Node` is either a File
// (leaf) or a Folder (internal node with children). Algorithms over
// the tree are `match` expressions that recurse on the Folder arm.
//
// The enum form is the default in Rust: closed variant set, zero
// vtable, exhaustive match forces you to handle both cases. For an
// open variant set (downstream plugins add new Node kinds) switch
// to a trait + Box<dyn Node>. See the notes at the bottom.

#[derive(Debug)]
pub enum Node {
    File { name: String, bytes: u64 },
    Folder { name: String, children: Vec<Node> },
}

impl Node {
    pub fn file(name: impl Into<String>, bytes: u64) -> Self {
        Node::File { name: name.into(), bytes }
    }
    pub fn folder(name: impl Into<String>, children: Vec<Node>) -> Self {
        Node::Folder { name: name.into(), children }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::File { name, .. } | Self::Folder { name, .. } => name,
        }
    }

    /// Recursive size: leaves report their bytes; folders sum over children.
    pub fn total_size(&self) -> u64 {
        match self {
            Self::File { bytes, .. } => *bytes,
            Self::Folder { children, .. } => children.iter().map(Self::total_size).sum(),
        }
    }

    /// Depth-first print with indentation.
    pub fn print(&self, depth: usize) {
        let pad = "  ".repeat(depth);
        match self {
            Self::File { name, bytes }       => println!("{pad}{name} ({bytes} B)"),
            Self::Folder { name, children }  => {
                println!("{pad}{name}/");
                for c in children { c.print(depth + 1); }
            }
        }
    }

    /// Generic visitor — apply `f` to every File in the tree.
    pub fn walk_files(&self, f: &mut impl FnMut(&str, u64)) {
        match self {
            Self::File { name, bytes }      => f(name, *bytes),
            Self::Folder { children, .. }   => {
                for c in children { c.walk_files(f); }
            }
        }
    }
}

fn main() {
    let root = Node::folder("project", vec![
        Node::folder("src", vec![
            Node::file("lib.rs", 5_000),
            Node::file("main.rs", 3_000),
        ]),
        Node::file("README.md", 2_000),
        Node::file("Cargo.toml", 1_000),
    ]);

    root.print(0);
    println!("\ntotal = {} bytes", root.total_size());

    let mut by_ext = std::collections::HashMap::<String, u64>::new();
    root.walk_files(&mut |name, bytes| {
        let ext = std::path::Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("(none)")
            .to_string();
        *by_ext.entry(ext).or_insert(0) += bytes;
    });
    let mut lines: Vec<_> = by_ext.iter().collect();
    lines.sort_by_key(|(k, _)| (*k).clone());
    println!("\nby extension:");
    for (ext, total) in lines { println!("  .{ext}: {total} B"); }
}
