#!/usr/bin/env node
// One-shot generator for the full pattern curriculum.
// Re-running is safe — stub-pattern.mjs skips any index.md that already exists.

import { stub } from './stub-pattern.mjs';

const PATTERNS = [
  // Track A — GoF Creational (builder is already fleshed out at seq 1)
  { track: 'gof', group: 'creational', seq: 2, slug: 'factory-method',   title: 'Factory Method',    gof_name: 'Factory Method',    verdict: 'use-with-caveats' },
  { track: 'gof', group: 'creational', seq: 3, slug: 'abstract-factory', title: 'Abstract Factory',  gof_name: 'Abstract Factory',  verdict: 'prefer-rust-alternative', rust_alternative: 'sealed-trait' },
  { track: 'gof', group: 'creational', seq: 4, slug: 'prototype',        title: 'Prototype',          gof_name: 'Prototype',         verdict: 'prefer-rust-alternative', rust_alternative: 'newtype' },
  { track: 'gof', group: 'creational', seq: 5, slug: 'singleton',        title: 'Singleton',          gof_name: 'Singleton',         verdict: 'use-with-caveats' },

  // Track A — GoF Structural
  { track: 'gof', group: 'structural', seq: 1, slug: 'adapter',    title: 'Adapter',    gof_name: 'Adapter',    verdict: 'use' },
  { track: 'gof', group: 'structural', seq: 2, slug: 'bridge',     title: 'Bridge',     gof_name: 'Bridge',     verdict: 'use-with-caveats' },
  { track: 'gof', group: 'structural', seq: 3, slug: 'composite',  title: 'Composite',  gof_name: 'Composite',  verdict: 'use' },
  { track: 'gof', group: 'structural', seq: 4, slug: 'decorator',  title: 'Decorator',  gof_name: 'Decorator',  verdict: 'use-with-caveats' },
  { track: 'gof', group: 'structural', seq: 5, slug: 'facade',     title: 'Facade',     gof_name: 'Facade',     verdict: 'use' },
  { track: 'gof', group: 'structural', seq: 6, slug: 'flyweight',  title: 'Flyweight',  gof_name: 'Flyweight',  verdict: 'use-with-caveats' },
  { track: 'gof', group: 'structural', seq: 7, slug: 'proxy',      title: 'Proxy',      gof_name: 'Proxy',      verdict: 'use-with-caveats' },

  // Track A — GoF Behavioral
  { track: 'gof', group: 'behavioral', seq: 1,  slug: 'chain-of-responsibility', title: 'Chain of Responsibility', gof_name: 'Chain of Responsibility', verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 2,  slug: 'command',         title: 'Command',         gof_name: 'Command',         verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 3,  slug: 'interpreter',     title: 'Interpreter',     gof_name: 'Interpreter',     verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 4,  slug: 'iterator',        title: 'Iterator',        gof_name: 'Iterator',        verdict: 'use' },
  { track: 'gof', group: 'behavioral', seq: 5,  slug: 'mediator',        title: 'Mediator',        gof_name: 'Mediator',        verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 6,  slug: 'memento',         title: 'Memento',         gof_name: 'Memento',         verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 7,  slug: 'observer',        title: 'Observer',        gof_name: 'Observer',        verdict: 'prefer-rust-alternative', rust_alternative: 'closure-as-callback' },
  { track: 'gof', group: 'behavioral', seq: 8,  slug: 'state',           title: 'State',           gof_name: 'State',           verdict: 'prefer-rust-alternative', rust_alternative: 'typestate' },
  { track: 'gof', group: 'behavioral', seq: 9,  slug: 'strategy',        title: 'Strategy',        gof_name: 'Strategy',        verdict: 'use' },
  { track: 'gof', group: 'behavioral', seq: 10, slug: 'template-method', title: 'Template Method', gof_name: 'Template Method', verdict: 'use-with-caveats' },
  { track: 'gof', group: 'behavioral', seq: 11, slug: 'visitor',         title: 'Visitor',         gof_name: 'Visitor',         verdict: 'use-with-caveats' },

  // Track B — Rust-Idiomatic (typestate is already fleshed out at seq 1)
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 2,  slug: 'newtype',                     title: 'Newtype',                       verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 3,  slug: 'raii-and-drop',               title: 'RAII & Drop',                   verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 4,  slug: 'sealed-trait',                title: 'Sealed Trait',                  verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 5,  slug: 'phantom-types',               title: 'Phantom Types',                 verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 6,  slug: 'interior-mutability',         title: 'Interior Mutability',           verdict: 'use-with-caveats' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 7,  slug: 'error-as-values',             title: 'Error-as-Values',               verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 8,  slug: 'builder-with-consuming-self', title: 'Builder with Consuming self',   verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 9,  slug: 'iterator-as-strategy',        title: 'Iterator as Strategy',          verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 10, slug: 'closure-as-callback',         title: 'Closure as Callback',           verdict: 'use' },
  { track: 'rust-idiomatic', group: 'rust-idiomatic', seq: 11, slug: 'from-into-conversions',       title: 'From / Into Conversions',       verdict: 'use' },
];

for (const p of PATTERNS) {
  await stub(p);
}

console.log(`\n[bulk-stub] processed ${PATTERNS.length} patterns.`);
