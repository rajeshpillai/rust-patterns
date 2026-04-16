import { For, Show, createMemo, createSignal, onCleanup, onMount } from 'solid-js';
import { A, useNavigate } from '@solidjs/router';

const VERDICT_CHIP = {
  'use':                     { icon: '✅', class: 'text-emerald-600 dark:text-emerald-400' },
  'use-with-caveats':        { icon: '⚠️', class: 'text-amber-600 dark:text-amber-400' },
  'prefer-rust-alternative': { icon: '♻️', class: 'text-sky-600 dark:text-sky-400' },
  'anti-pattern-in-rust':    { icon: '❌', class: 'text-rose-600 dark:text-rose-400' },
};

const TRACKS = [
  { id: 'gof', label: 'Gang of Four' },
  { id: 'rust-idiomatic', label: 'Rust-Idiomatic' },
];

function groupLabel(group) {
  switch (group) {
    case 'creational':     return 'Creational';
    case 'structural':     return 'Structural';
    case 'behavioral':     return 'Behavioral';
    case 'rust-idiomatic': return 'Rust-Idiomatic';
    default:               return group;
  }
}

export default function Sidebar(props) {
  const [query, setQuery] = createSignal('');
  let searchInput;

  // Keyboard shortcut: "/" focuses the search box, Esc clears it.
  onMount(() => {
    const onKey = (e) => {
      const tag = (e.target?.tagName ?? '').toLowerCase();
      if (e.key === '/' && tag !== 'input' && tag !== 'textarea') {
        e.preventDefault();
        searchInput?.focus();
        searchInput?.select();
      } else if (e.key === 'Escape' && document.activeElement === searchInput) {
        setQuery('');
        searchInput?.blur();
      }
    };
    window.addEventListener('keydown', onKey);
    onCleanup(() => window.removeEventListener('keydown', onKey));
  });

  const filtered = createMemo(() => {
    const q = query().trim().toLowerCase();
    if (!q) return props.patterns;
    return props.patterns.filter((p) => {
      const hay = [p.title, p.gof_name, p.id, p.group, p.verdict]
        .filter(Boolean)
        .join(' ')
        .toLowerCase();
      return hay.includes(q);
    });
  });

  const grouped = createMemo(() => {
    const byGroup = new Map();
    for (const p of filtered()) {
      const g = p.group ?? 'other';
      if (!byGroup.has(g)) byGroup.set(g, []);
      byGroup.get(g).push(p);
    }
    return Array.from(byGroup.entries());
  });

  return (
    <aside class="w-72 shrink-0 border-r border-neutral-200 dark:border-neutral-800 bg-neutral-50 dark:bg-neutral-900 overflow-y-auto flex flex-col">
      <div class="px-4 py-3 border-b border-neutral-200 dark:border-neutral-800 flex items-center justify-between">
        <div class="min-w-0">
          <A href="/" class="text-xs text-neutral-500 hover:underline">← home</A>
          <h2 class="text-sm font-semibold mt-1 truncate">
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

      <div class="px-3 py-2 border-b border-neutral-200 dark:border-neutral-800 space-y-2">
        <div class="flex gap-1 text-xs">
          <For each={TRACKS}>
            {(t) => {
              const active = () => props.track === t.id;
              return (
                <A
                  href={`/patterns/${t.id}`}
                  class={`flex-1 text-center px-2 py-1 rounded ${
                    active()
                      ? 'bg-amber-500 text-white'
                      : 'bg-neutral-200 dark:bg-neutral-800 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-700'
                  }`}
                >
                  {t.label}
                </A>
              );
            }}
          </For>
        </div>

        <div class="relative">
          <input
            ref={searchInput}
            type="search"
            value={query()}
            onInput={(e) => setQuery(e.currentTarget.value)}
            placeholder="Search patterns…"
            aria-label="Search patterns"
            class="w-full pl-8 pr-8 py-1.5 text-sm rounded bg-white dark:bg-neutral-800 border border-neutral-300 dark:border-neutral-700 focus:outline-none focus:ring-2 focus:ring-amber-500"
          />
          <svg
            class="absolute left-2 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="11" cy="11" r="7" />
            <path d="M21 21l-4.3-4.3" />
          </svg>
          <Show
            when={query()}
            fallback={
              <kbd class="absolute right-2 top-1/2 -translate-y-1/2 text-[10px] px-1.5 py-0.5 rounded border border-neutral-300 dark:border-neutral-700 text-neutral-500">
                /
              </kbd>
            }
          >
            <button
              class="absolute right-2 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200"
              onClick={() => setQuery('')}
              aria-label="Clear search"
            >
              ✕
            </button>
          </Show>
        </div>
      </div>

      <nav class="flex-1 px-2 py-3 space-y-5">
        <Show
          when={filtered().length > 0}
          fallback={
            <p class="px-3 py-6 text-sm text-neutral-500">
              No patterns match <span class="font-mono">"{query()}"</span>.
            </p>
          }
        >
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
        </Show>
      </nav>

      <div class="px-3 py-2 border-t border-neutral-200 dark:border-neutral-800 text-[11px] text-neutral-500">
        Press <kbd class="px-1 rounded border border-neutral-300 dark:border-neutral-700">/</kbd> to search ·
        <kbd class="ml-1 px-1 rounded border border-neutral-300 dark:border-neutral-700">Esc</kbd> to clear
      </div>
    </aside>
  );
}
