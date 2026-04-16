#!/usr/bin/env node
// Generate a stub index.md for a pattern folder.
//
// Usage:
//   node scripts/stub-pattern.mjs <track> <group> <seq> <slug> <title> [gof_name] [verdict] [rust_alternative]
//
// Example:
//   node scripts/stub-pattern.mjs gof creational 2 factory-method "Factory Method" "Factory Method" use-with-caveats
//
// Writes patterns/<group_folder>/<slug>/index.md and creates empty
// diagrams/ and code/ subdirectories. Never overwrites an existing
// index.md — existing work is preserved.

import { writeFile, mkdir } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

function groupFolder(track, group) {
  if (track === 'gof') return `gof-${group}`;
  return 'rust-idiomatic';
}

async function stub({ track, group, seq, slug, title, gof_name, verdict, rust_alternative }) {
  const dir = join(repoRoot, 'patterns', groupFolder(track, group), slug);
  await mkdir(join(dir, 'diagrams'), { recursive: true });
  await mkdir(join(dir, 'code'), { recursive: true });

  const indexPath = join(dir, 'index.md');
  if (existsSync(indexPath)) {
    console.log(`[stub] skip (already exists): ${indexPath}`);
    return;
  }

  const gofLine = gof_name ? `gof_name: ${gof_name}` : 'gof_name: null';
  const altLine = rust_alternative
    ? `rust_alternative: ${rust_alternative}`
    : 'rust_alternative: null';

  const body = `---
id: ${slug}
track: ${track}
group: ${group}
sequence: ${seq}
title: ${title}
${gofLine}
verdict: ${verdict}
${altLine}
hints: []
---

## Intent

> This pattern is stubbed. A full treatment is pending — see the authoring
> checklist in [CLAUDE.md](/CLAUDE.md) for the required sections, diagrams,
> and code artifacts.

## Status

- [ ] Intent written
- [ ] Motivation + concept diagram
- [ ] Classical form (Track A only) + class diagram
- [ ] Idiomatic Rust form + Playground link
- [ ] Anti-patterns & caveats
- [ ] Compiler-error walkthrough
- [ ] Verdict justified
- [ ] Related patterns linked

## Contribute

Copy the shape of [patterns/gof-creational/builder/](/patterns/gof-creational/builder/)
for a Track A example, or [patterns/rust-idiomatic/typestate/](/patterns/rust-idiomatic/typestate/)
for a Track B example.
`;

  await writeFile(indexPath, body, 'utf8');
  console.log(`[stub] wrote ${indexPath}`);
}

async function run() {
  const args = process.argv.slice(2);
  if (args.length < 5) {
    console.error('Usage: node scripts/stub-pattern.mjs <track> <group> <seq> <slug> <title> [gof_name] [verdict] [rust_alternative]');
    process.exit(1);
  }
  const [track, group, seq, slug, title, gof_name, verdict, rust_alternative] = args;
  await stub({
    track,
    group,
    seq: Number(seq),
    slug,
    title,
    gof_name: gof_name || null,
    verdict: verdict || 'use-with-caveats',
    rust_alternative: rust_alternative || null,
  });
}

// If imported as a module, export `stub` for bulk use; otherwise run CLI.
if (import.meta.url === `file://${process.argv[1]}`) {
  run().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}

export { stub };
