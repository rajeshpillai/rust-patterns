# CLAUDE.md — Rust Design Patterns Tutorial Site

A statically hosted (GitHub Pages) site that teaches design patterns **the Rust way**: classical Gang of Four patterns translated into idiomatic Rust, plus a second track of Rust-native patterns (typestate, newtype, RAII, sealed traits, phantom types, error-as-values, etc.). Visual-first — every pattern ships with diagrams, runnable code, and a compiler-error walkthrough.

---

## Mission

> Teach patterns as applied ownership. When the borrow checker disagrees with GoF, the borrow checker wins.

Every pattern in this site must be evaluated against one question before it is recommended:

> **Does this survive ownership, borrowing, and zero-cost abstractions?**

If yes, teach it. If no, teach why it fails and what Rust offers instead. Many GoF patterns are *anti-patterns* in Rust — that must be stated loudly, not softened.

---

## Dual-Track Curriculum

### Track A — Gang of Four (23 patterns)

Each pattern appears with three forms side-by-side: classical GoF, a direct Rust translation (usually ugly), and the idiomatic Rust rewrite. A **Verdict** chip on every pattern tells the learner at a glance whether it survives translation.

#### Creational
1. Builder
2. Factory Method
3. Abstract Factory
4. Prototype
5. Singleton

#### Structural
6. Adapter
7. Bridge
8. Composite
9. Decorator
10. Facade
11. Flyweight
12. Proxy

#### Behavioral
13. Chain of Responsibility
14. Command
15. Interpreter
16. Iterator
17. Mediator
18. Memento
19. Observer
20. State
21. Strategy
22. Template Method
23. Visitor

### Track B — Rust-Idiomatic Patterns

Patterns GoF never covered, sequenced **after** the GoF track so learners first understand the classical vocabulary they are replacing.

1. Typestate
2. Newtype
3. RAII & `Drop`
4. Sealed Trait
5. Phantom Types (`PhantomData`)
6. Interior Mutability (`Cell`, `RefCell`, `OnceCell`, `Mutex`, `RwLock`)
7. Error-as-Values (`thiserror` for libraries, `anyhow` for binaries)
8. Builder with Consuming `self`
9. Iterator as Strategy
10. Closure as Callback (`Fn` / `FnMut` / `FnOnce`)
11. `From` / `Into` Conversions and `TryFrom` for Fallible Ones

---

## Repository Layout (MANDATORY)

```
patterns/rust/
├── CLAUDE.md                      ← this file
├── README.md                      ← public-facing blurb + live deploy link
├── patterns/                      ← all tutorial content, one folder per pattern
│   ├── gof-creational/
│   │   └── builder/
│   │       ├── index.md           ← the tutorial (frontmatter + prose)
│   │       ├── diagrams/
│   │       │   ├── class.mmd      ← Mermaid class diagram (GoF form)
│   │       │   ├── rust-form.mmd  ← Mermaid diagram of Rust adaptation
│   │       │   ├── sequence.mmd   ← Mermaid sequence/state diagram
│   │       │   └── concept.svg    ← Excalidraw-exported conceptual sketch
│   │       │   └── concept.excalidraw  ← Excalidraw source, committed
│   │       └── code/
│   │           ├── gof-style.rs   ← direct GoF translation (often anti-pattern in Rust)
│   │           ├── idiomatic.rs   ← idiomatic Rust rewrite
│   │           └── broken.rs      ← common misuse with compiler-error walkthrough
│   ├── gof-structural/
│   ├── gof-behavioral/
│   └── rust-idiomatic/
├── frontend/                      ← SolidJS + Vite + Tailwind v4 site
├── scripts/
│   └── build-index.mjs            ← walks patterns/**, emits content-index.js
└── .github/workflows/
    └── deploy.yml                 ← builds on push, publishes to GitHub Pages
```

**Rule:** every pattern is a self-contained folder. Prose, diagrams, code, and Excalidraw sources live together so a PR reviewer sees one atomic unit.

---

## File & Folder Conventions

- All files and folders are **lowercase-hyphenated**. No camelCase, no snake_case, no spaces.
- Pattern folders use their common GoF name for Track A (`builder`, `abstract-factory`) and their Rust-community name for Track B (`typestate`, `newtype`).
- Binary assets are forbidden except exported `.svg` (and `.excalidraw` source, which is JSON).

### `index.md` frontmatter schema

```yaml
---
id: builder                     # stable slug, unique across the site
track: gof                      # "gof" | "rust-idiomatic"
group: creational               # creational | structural | behavioral | rust-idiomatic
sequence: 1                     # order within group
title: Builder
gof_name: Builder               # null for Track B
verdict: use-with-caveats       # use | use-with-caveats | prefer-rust-alternative | anti-pattern-in-rust
rust_alternative: null          # slug of the pattern to prefer, when applicable
playground_links:
  gof-style: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=...
  idiomatic: https://play.rust-lang.org/...
  broken:    https://play.rust-lang.org/...
hints:
  - What does ownership say about fluent setters?
  - When should the builder consume itself vs. borrow?
---
```

### `index.md` section template (MANDATORY)

Every pattern's `index.md` must include these sections, in order:

1. **Intent** — one-paragraph summary.
2. **Problem / Motivation** — includes a Mermaid sequence or concept diagram.
3. **Classical GoF Form** (Track A only) — Mermaid class diagram + minimal example.
4. **Why GoF Translates Poorly to Rust** — call out the specific ownership, borrowing, or dispatch tension.
5. **Idiomatic Rust Form** — Mermaid class + sequence diagram + Playground link.
6. **Anti-patterns & Rust-specific caveats** — see Idiomatic-Rust Rules below.
7. **Compiler-Error Walkthrough** — paste the broken version's error, including the `E####` code, and decode it line-by-line.
8. **When to Reach for This Pattern (and When NOT to)** — decision guidance.
9. **Related Patterns & Next Steps** — links to sibling pattern folders.

---

## Diagramming Rules

- **Mermaid** for UML class, sequence, and state diagrams. Committed as `.mmd` and rendered inline in the frontend.
- **Excalidraw** for conceptual/hand-drawn diagrams. Commit **both** the `.excalidraw` source (so diagrams remain editable) and the exported `.svg`.
- **Every** pattern must ship **at least one class/structure diagram AND at least one behavioral diagram** (sequence or state). Structure-only is not enough — patterns are defined by their dynamics.
- No PNG/JPG screenshots of diagrams. The only binary-ish blob allowed is exported `.svg`.
- Diagram files live in `diagrams/` within each pattern folder — never in a global assets directory.

---

## Idiomatic-Rust Rules (the "no blind `unwrap`" clause)

These rules apply to every code snippet shipped in the site. Drafts that break them get rewritten before merge.

- **No `unwrap()` / `expect()`** in example code unless the section is explicitly teaching panics — and then it must be flagged with a ⚠️ callout explaining why it's acceptable (or why it isn't). `expect` with a meaningful message is preferable to `unwrap` when a panic is genuinely the right behavior.
- **Prefer `?` and `Result<T, E>`** with a typed error enum. Show `thiserror` for libraries, `anyhow` for binaries. Never return `Box<dyn Error>` from a library's public API.
- **No `clone()` as a borrow-checker workaround** without a comment justifying the allocation cost. If cloning is the right answer, say so.
- **API boundaries:** accept `&str` over `String`, `&[T]` over `&Vec<T>`, `impl AsRef<Path>` over `&Path`. Accept the most borrowed form you can.
- **Shared state is a last resort.** Show the ownership-first design before reaching for `Rc<RefCell<T>>` or `Arc<Mutex<T>>`. When the shared-state form is necessary, name the tradeoff.
- **Enums over trait objects** for closed sets of variants. Reach for `dyn Trait` only when the set is genuinely open and dynamic dispatch is acceptable.
- **Use `#[must_use]`, `#[non_exhaustive]`, and sealed-trait patterns** wherever they apply. Compile-time guarantees beat runtime checks.
- **No `unsafe`** unless the section is Track B's unsafe-adjacent material, and even then only to *create a safe boundary*. The `unsafe` block must be accompanied by the invariants it relies on.
- **Clippy-clean.** Every public snippet must survive `cargo clippy -- -D warnings`. The site's CI enforces this.

---

## Rust Playground Integration

GitHub Pages cannot execute Rust. We do not ship a backend. We delegate execution to [play.rust-lang.org](https://play.rust-lang.org).

- Every runnable snippet has a shareable Playground URL recorded in the frontmatter `playground_links` map under the same key as the file name (`gof-style` → `code/gof-style.rs`).
- The frontend renders an **Open in Playground** button next to each code block, driven by `content-index.js`.
- Playground URLs are stable gist-backed links. When a snippet changes, re-upload to Playground and update the URL in the same PR — never let them drift.

---

## Frontend Conventions

Stack: **SolidJS + Vite + Tailwind v4 + CodeMirror 6** (mirrors [`../../rust-katas/frontend/package.json`](../../rust-katas/frontend/package.json)).

- **Landing page**: two cards — **Gang of Four** and **Rust-Idiomatic Patterns** — each linking into the pattern browser filtered by track.
- **Sidebar**: collapsible, hamburger menu on mobile. Entries show `sequence. title` and a verdict chip (✅ use, ⚠️ use-with-caveats, ♻️ prefer-rust-alternative, ❌ anti-pattern-in-rust).
- **Pattern view**: three resizable panes — prose, diagram, code. Each pane has a maximize button.
- **Theme**: light/dark toggle, persisted in `localStorage`.
- **Routing**: client-side hash routing (`#/patterns/builder`) — GitHub Pages does not need SPA rewrites, and a `404.html` copy of `index.html` handles deep-link refreshes.
- **Component reuse**: pattern the sidebar, code panel, theme toggle, and workspace after [`../../rust-katas/frontend/src/components/`](../../rust-katas/frontend/src/components/). Do not reinvent.

---

## Build & Deploy (GitHub Pages)

- `scripts/build-index.mjs` walks `patterns/**/index.md`, parses frontmatter, and emits `frontend/src/content-index.js` plus a search index. Wired as `prebuild` in `frontend/package.json`.
- `vite build` emits `frontend/dist/`.
- `vite.config.js` sets `base: '/<repo-name>/'` for a project-site deploy. Update this if the repo is renamed.
- `.github/workflows/deploy.yml` uses `actions/deploy-pages@v4`, runs on push to `main`, and publishes `frontend/dist`.
- Copy `index.html` to `404.html` in the build step so SPA refreshes on deep links don't 404.

---

## Authoring Rules (MANDATORY for every new pattern)

Every pattern added to the site must ship with:

- ❌ a broken version (`code/broken.rs`)
- ✅ a correct idiomatic version (`code/idiomatic.rs`)
- 🔁 for Track A only, a direct GoF-style translation (`code/gof-style.rs`) — even when it's an anti-pattern, especially then
- 🧠 an explanation of the invariant Rust enforces (or refuses to enforce)
- 🔍 a compiler-error walkthrough with the `E####` code decoded
- 🎨 at least one class/structure diagram **and** one behavioral diagram
- 🔗 a Rust Playground link per runnable snippet, recorded in frontmatter
- ⚖️ an explicit **Verdict** block: `use` / `use-with-caveats` / `prefer-rust-alternative` / `anti-pattern-in-rust`

A pattern missing any of these is not ready to merge.

---

## Teaching Rules

You must:
- Explain *why* something fails. Paste the compiler error verbatim.
- Treat the borrow checker as a collaborator. When it refuses, it is teaching you about an invariant.
- Name the invariant being enforced (or violated) before reaching for a fix.
- Build intuition before optimization. Performance claims require measurement, not vibes.
- Prefer compile-time guarantees (typestate, sealed traits, phantom types) over runtime checks (`RefCell`, dynamic dispatch) when both are viable.

You must NOT:
- Hide or paraphrase compiler errors.
- Recommend `unwrap()`, `clone()`, `Rc<RefCell<T>>`, or `unsafe` as default solutions.
- Present a GoF pattern without questioning whether it survives translation.
- Skip the diagrams because the prose "already explains it." Visual and textual reinforce different mental models.

---

## Success Criteria

A learner finishing this site can:
- Recognize when a GoF pattern is the wrong tool in Rust.
- Name the idiomatic Rust alternative and justify the choice.
- Implement the pattern without reflexively reaching for `unwrap`, `clone`, `RefCell`, or `unsafe`.
- Read the compiler error that results from the broken form and explain both the `E####` code and the invariant it protects.
- Draw the class and sequence diagrams for the pattern from memory.

---

## Final Instruction

Teach patterns as applied ownership.
When the borrow checker disagrees with GoF, the borrow checker wins.
Explain everything. Paste the compiler error. Draw the diagram.
Proceed deliberately.
