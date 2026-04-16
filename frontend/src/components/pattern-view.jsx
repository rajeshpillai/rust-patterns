import { For, Show, onMount, onCleanup, createEffect } from 'solid-js';
import { A, useNavigate } from '@solidjs/router';
import Markdown from './markdown.jsx';
import CodePanel from './code-panel.jsx';

const VERDICT_META = {
  'use':                     { icon: '✅', label: 'Use', color: 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' },
  'use-with-caveats':        { icon: '⚠️', label: 'Use with caveats', color: 'bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300' },
  'prefer-rust-alternative': { icon: '♻️', label: 'Prefer Rust alternative', color: 'bg-sky-100 text-sky-800 dark:bg-sky-950 dark:text-sky-300' },
  'anti-pattern-in-rust':    { icon: '❌', label: 'Anti-pattern in Rust', color: 'bg-rose-100 text-rose-800 dark:bg-rose-950 dark:text-rose-300' },
};

export default function PatternView(props) {
  const verdict = () => VERDICT_META[props.pattern?.verdict] ?? VERDICT_META['use'];
  const navigate = useNavigate();

  // Scroll to top when the selected pattern changes so readers don't
  // start mid-article after clicking a sidebar entry.
  let scroller;
  createEffect(() => {
    void props.pattern?.id;
    if (scroller) scroller.scrollTop = 0;
  });

  // Keyboard shortcuts: j / → next, k ← → previous, skipping while
  // the user is typing in an input or search box.
  onMount(() => {
    const onKey = (e) => {
      const tag = (e.target?.tagName ?? '').toLowerCase();
      if (tag === 'input' || tag === 'textarea') return;
      if (e.metaKey || e.ctrlKey || e.altKey) return;
      if ((e.key === 'j' || e.key === 'ArrowRight') && props.next) {
        e.preventDefault();
        navigate(`/patterns/${props.track}/${props.next.id}`);
      } else if ((e.key === 'k' || e.key === 'ArrowLeft') && props.prev) {
        e.preventDefault();
        navigate(`/patterns/${props.track}/${props.prev.id}`);
      }
    };
    window.addEventListener('keydown', onKey);
    onCleanup(() => window.removeEventListener('keydown', onKey));
  });

  return (
    <div ref={scroller} class="flex-1 overflow-y-auto">
      <article class="max-w-4xl mx-auto px-6 py-8">
        <header class="mb-6">
          <div class="text-xs text-neutral-500 uppercase tracking-wider">
            {props.pattern.group ?? props.pattern.track}
          </div>
          <h1 class="text-3xl font-semibold tracking-tight mt-1">{props.pattern.title}</h1>
          <div class="mt-3 flex flex-wrap items-center gap-2">
            <span class={`px-2.5 py-1 rounded-full text-xs font-medium ${verdict().color}`}>
              {verdict().icon} {verdict().label}
            </span>
            <Show when={props.pattern.gof_name}>
              <span class="px-2.5 py-1 rounded-full text-xs bg-neutral-200 dark:bg-neutral-800 text-neutral-700 dark:text-neutral-300">
                GoF: {props.pattern.gof_name}
              </span>
            </Show>
          </div>
        </header>

        <Markdown source={props.pattern.markdown} />

        <Show when={props.pattern.codeFiles?.length}>
          <section class="mt-10 border-t border-neutral-200 dark:border-neutral-800 pt-6">
            <h2 class="text-xl font-semibold mb-4">Code</h2>
            <For each={props.pattern.codeFiles}>
              {(file) => (
                <CodePanel
                  filename={file.filename}
                  source={file.source}
                  playground={file.playground}
                />
              )}
            </For>
          </section>
        </Show>

        <nav
          aria-label="Pattern navigation"
          class="mt-12 pt-6 border-t border-neutral-200 dark:border-neutral-800 grid grid-cols-2 gap-4"
        >
          <Show
            when={props.prev}
            fallback={<span class="opacity-0 select-none" aria-hidden />}
          >
            <A
              href={`/patterns/${props.track}/${props.prev.id}`}
              class="group block rounded-lg border border-neutral-200 dark:border-neutral-800 p-4 hover:border-amber-500 transition"
            >
              <div class="text-[11px] uppercase tracking-wider text-neutral-500">
                ← Previous
              </div>
              <div class="text-sm font-semibold mt-1 group-hover:text-amber-600 dark:group-hover:text-amber-400">
                {props.prev.title}
              </div>
              <div class="text-xs text-neutral-500 mt-0.5">
                {props.prev.group}
              </div>
            </A>
          </Show>

          <Show
            when={props.next}
            fallback={<span class="opacity-0 select-none" aria-hidden />}
          >
            <A
              href={`/patterns/${props.track}/${props.next.id}`}
              class="group block rounded-lg border border-neutral-200 dark:border-neutral-800 p-4 hover:border-amber-500 transition text-right"
            >
              <div class="text-[11px] uppercase tracking-wider text-neutral-500">
                Next →
              </div>
              <div class="text-sm font-semibold mt-1 group-hover:text-amber-600 dark:group-hover:text-amber-400">
                {props.next.title}
              </div>
              <div class="text-xs text-neutral-500 mt-0.5">
                {props.next.group}
              </div>
            </A>
          </Show>
        </nav>

        <p class="mt-4 text-[11px] text-neutral-500 text-center">
          Press <kbd class="px-1 rounded border border-neutral-300 dark:border-neutral-700">j</kbd> / <kbd class="px-1 rounded border border-neutral-300 dark:border-neutral-700">→</kbd> for next ·
          <kbd class="ml-1 px-1 rounded border border-neutral-300 dark:border-neutral-700">k</kbd> / <kbd class="px-1 rounded border border-neutral-300 dark:border-neutral-700">←</kbd> for previous
        </p>
      </article>
    </div>
  );
}
