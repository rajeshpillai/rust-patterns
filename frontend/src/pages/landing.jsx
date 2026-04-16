import { A } from '@solidjs/router';
import ThemeToggle from '../components/theme-toggle.jsx';
import patterns from '../generated/patterns.js';

function countByTrack(track) {
  return patterns.filter((p) => p.track === track).length;
}

export default function Landing() {
  const gofCount = countByTrack('gof');
  const idiomaticCount = countByTrack('rust-idiomatic');

  return (
    <div class="min-h-full">
      <header class="flex items-center justify-between px-6 py-4 border-b border-neutral-200 dark:border-neutral-800">
        <h1 class="text-lg font-semibold tracking-tight">Rust Design Patterns</h1>
        <ThemeToggle />
      </header>

      <main class="max-w-5xl mx-auto px-6 py-12">
        <p class="text-neutral-600 dark:text-neutral-300 text-lg max-w-2xl mb-10">
          Patterns the Rust way. GoF translated honestly — including when the honest answer is
          <em> "don't do this in Rust, do that instead."</em>
        </p>

        <div class="grid gap-6 md:grid-cols-2">
          <A
            href="/patterns/gof"
            class="block rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900 p-6 hover:border-amber-500 hover:shadow-lg transition"
          >
            <div class="flex items-baseline justify-between">
              <h2 class="text-xl font-semibold">Gang of Four</h2>
              <span class="text-xs text-neutral-500">{gofCount} patterns</span>
            </div>
            <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-300">
              The classical 23: Creational, Structural, Behavioral. Each with a GoF-style
              translation, an idiomatic Rust rewrite, and a Verdict.
            </p>
            <div class="mt-4 flex flex-wrap gap-2 text-xs">
              <span class="px-2 py-1 rounded-full bg-emerald-100 dark:bg-emerald-950 text-emerald-800 dark:text-emerald-300">✅ use</span>
              <span class="px-2 py-1 rounded-full bg-amber-100 dark:bg-amber-950 text-amber-800 dark:text-amber-300">⚠️ caveats</span>
              <span class="px-2 py-1 rounded-full bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-300">♻️ prefer Rust alt</span>
              <span class="px-2 py-1 rounded-full bg-rose-100 dark:bg-rose-950 text-rose-800 dark:text-rose-300">❌ anti-pattern</span>
            </div>
          </A>

          <A
            href="/patterns/rust-idiomatic"
            class="block rounded-xl border border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-900 p-6 hover:border-amber-500 hover:shadow-lg transition"
          >
            <div class="flex items-baseline justify-between">
              <h2 class="text-xl font-semibold">Rust-Idiomatic Patterns</h2>
              <span class="text-xs text-neutral-500">{idiomaticCount} patterns</span>
            </div>
            <p class="mt-2 text-sm text-neutral-600 dark:text-neutral-300">
              Patterns GoF never covered: typestate, newtype, RAII, sealed traits, phantom
              types, interior mutability, and error-as-values.
            </p>
            <ul class="mt-4 text-xs text-neutral-500 dark:text-neutral-400 grid grid-cols-2 gap-y-1">
              <li>• Typestate</li>
              <li>• Newtype</li>
              <li>• RAII &amp; Drop</li>
              <li>• Sealed Trait</li>
              <li>• Phantom Types</li>
              <li>• Error-as-Values</li>
            </ul>
          </A>
        </div>

        <footer class="mt-16 text-xs text-neutral-500 dark:text-neutral-500">
          Runnable examples link to <a class="underline" href="https://play.rust-lang.org">play.rust-lang.org</a>.
          Source at <code>patterns/&lt;track&gt;/&lt;pattern&gt;/</code>.
        </footer>
      </main>
    </div>
  );
}
