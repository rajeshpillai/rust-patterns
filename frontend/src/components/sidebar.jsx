import { For, createMemo } from 'solid-js';
import { A } from '@solidjs/router';

const VERDICT_CHIP = {
  'use': { icon: '✅', class: 'text-emerald-600 dark:text-emerald-400' },
  'use-with-caveats': { icon: '⚠️', class: 'text-amber-600 dark:text-amber-400' },
  'prefer-rust-alternative': { icon: '♻️', class: 'text-sky-600 dark:text-sky-400' },
  'anti-pattern-in-rust': { icon: '❌', class: 'text-rose-600 dark:text-rose-400' },
};

function groupLabel(group) {
  switch (group) {
    case 'creational': return 'Creational';
    case 'structural': return 'Structural';
    case 'behavioral': return 'Behavioral';
    case 'rust-idiomatic': return 'Rust-Idiomatic';
    default: return group;
  }
}

export default function Sidebar(props) {
  const grouped = createMemo(() => {
    const byGroup = new Map();
    for (const p of props.patterns) {
      const g = p.group ?? 'other';
      if (!byGroup.has(g)) byGroup.set(g, []);
      byGroup.get(g).push(p);
    }
    return Array.from(byGroup.entries());
  });

  return (
    <aside class="w-72 shrink-0 border-r border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 overflow-y-auto">
      <div class="px-4 py-4 flex items-center justify-between border-b border-neutral-200 dark:border-neutral-800">
        <div>
          <A href="/" class="text-xs text-neutral-500 hover:underline">← home</A>
          <h2 class="text-sm font-semibold mt-1">
            {props.track === 'gof' ? 'Gang of Four' : 'Rust-Idiomatic'}
          </h2>
        </div>
        <button
          class="p-1 rounded hover:bg-neutral-200 dark:hover:bg-neutral-800 text-neutral-500"
          onClick={props.onClose}
          aria-label="Close sidebar"
          title="Hide sidebar"
        >
          ✕
        </button>
      </div>

      <nav class="px-2 py-3 space-y-5">
        <For each={grouped()}>
          {([group, list]) => (
            <div>
              <h3 class="px-2 pb-1 text-[11px] uppercase tracking-wider text-neutral-500">
                {groupLabel(group)}
              </h3>
              <ul class="space-y-0.5">
                <For each={list}>
                  {(p) => {
                    const v = VERDICT_CHIP[p.verdict] ?? VERDICT_CHIP['use'];
                    const active = () => p.id === props.activeId;
                    return (
                      <li>
                        <A
                          href={`/patterns/${props.track}/${p.id}`}
                          class={`flex items-center gap-2 px-2 py-1.5 rounded text-sm ${
                            active()
                              ? 'bg-amber-100 dark:bg-amber-950/50 text-amber-900 dark:text-amber-100'
                              : 'hover:bg-neutral-200 dark:hover:bg-neutral-800'
                          }`}
                        >
                          <span class="w-6 text-right text-xs text-neutral-500 tabular-nums">
                            {p.sequence}
                          </span>
                          <span class={`text-xs ${v.class}`} title={p.verdict}>{v.icon}</span>
                          <span class="truncate">{p.title}</span>
                        </A>
                      </li>
                    );
                  }}
                </For>
              </ul>
            </div>
          )}
        </For>
      </nav>
    </aside>
  );
}
