import { For, Show } from 'solid-js';
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

  return (
    <div class="flex-1 overflow-y-auto">
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
      </article>
    </div>
  );
}
