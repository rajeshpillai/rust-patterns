# Rust Design Patterns

**Live site:** https://rajeshpillai.github.io/rust-patterns/

A visual, static-hosted tutorial that teaches design patterns **the Rust way** — classical Gang of Four patterns translated into idiomatic Rust, plus a second track of Rust-native patterns (typestate, newtype, RAII, sealed traits, phantom types, error-as-values).

Every pattern ships with:

- A Mermaid class/structure diagram
- A Mermaid sequence or state diagram showing runtime behavior
- A broken example, an idiomatic example, and (for GoF) a direct translation
- A compiler-error walkthrough with the `E####` code decoded
- A **Verdict** — `use`, `use-with-caveats`, `prefer-rust-alternative`, or `anti-pattern-in-rust`
- An **Open in Rust Playground** link for every runnable snippet

## Stack

- **Content:** Markdown + Mermaid + Excalidraw in `patterns/`, one folder per pattern
- **Frontend:** SolidJS + Vite + Tailwind v4 + CodeMirror 6
- **Hosting:** GitHub Pages (fully static, no backend)
- **Execution:** Rust Playground links (play.rust-lang.org)

## Repository Layout

```
patterns/    one folder per pattern (index.md + diagrams/ + code/)
frontend/    SolidJS + Vite site
scripts/     build-index.mjs, deploy-gh-pages.sh, stub helpers
CLAUDE.md    authoring & teaching rules (read this before contributing)
```

## Local Development

```bash
cd frontend
npm install
npm run dev        # starts Vite on http://localhost:5173
npm run build      # emits frontend/dist/ for GitHub Pages
```

## Publishing

Deploy to GitHub Pages from the repo root:

```bash
./scripts/deploy-gh-pages.sh            # interactive — prompts before push
./scripts/deploy-gh-pages.sh --dry-run  # build + package without pushing
./scripts/deploy-gh-pages.sh --yes      # CI-friendly, no prompt
```

The script builds with `GITHUB_PAGES=true`, adds `.nojekyll` and a
`404.html` SPA fallback, and force-pushes `frontend/dist/` to the
`gh-pages` branch of `origin`. First-time setup requires enabling
Pages in the repo's Settings → Pages (source: `gh-pages` / root).

## Contributing a Pattern

1. Read [CLAUDE.md](./CLAUDE.md) — it is the contract.
2. Copy `patterns/gof-creational/builder/` as a template.
3. Fill in `index.md` frontmatter, the nine section template, and diagrams.
4. Upload runnable snippets to the Rust Playground and paste the URLs into frontmatter.
5. Submit a PR. CI runs `cargo clippy -- -D warnings` on every snippet.

## License

MIT. See [LICENSE](./LICENSE).
