import { createMemo, createSignal, Show } from 'solid-js';
import { useParams } from '@solidjs/router';
import patterns from '../generated/patterns.js';
import Sidebar from '../components/sidebar.jsx';
import PatternView from '../components/pattern-view.jsx';
import ThemeToggle from '../components/theme-toggle.jsx';

export default function PatternBrowser() {
  const params = useParams();
  const [sidebarOpen, setSidebarOpen] = createSignal(true);

  const activeTrack = createMemo(() => params.track ?? 'gof');

  const trackPatterns = createMemo(() =>
    patterns
      .filter((p) => p.track === activeTrack())
      .sort((a, b) => {
        if (a.group !== b.group) return (a.group ?? '').localeCompare(b.group ?? '');
        return (a.sequence ?? 0) - (b.sequence ?? 0);
      }),
  );

  const selected = createMemo(() => {
    const id = params.id ?? trackPatterns()[0]?.id;
    return trackPatterns().find((p) => p.id === id);
  });

  const neighbors = createMemo(() => {
    const list = trackPatterns();
    const sel = selected();
    if (!sel) return { prev: null, next: null };
    const i = list.findIndex((p) => p.id === sel.id);
    return {
      prev: i > 0 ? list[i - 1] : null,
      next: i >= 0 && i < list.length - 1 ? list[i + 1] : null,
    };
  });

  return (
    <div class="flex h-full">
      <Show when={sidebarOpen()}>
        <Sidebar
          track={activeTrack()}
          patterns={trackPatterns()}
          activeId={selected()?.id}
          onClose={() => setSidebarOpen(false)}
        />
      </Show>

      <div class="flex-1 flex flex-col min-w-0">
        <header class="flex items-center justify-between px-4 py-3 border-b border-neutral-200 dark:border-neutral-800">
          <button
            class="p-2 rounded hover:bg-neutral-200 dark:hover:bg-neutral-800"
            onClick={() => setSidebarOpen(!sidebarOpen())}
            aria-label="Toggle sidebar"
          >
            <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
              <rect x="2" y="4" width="16" height="2" rx="1" />
              <rect x="2" y="9" width="16" height="2" rx="1" />
              <rect x="2" y="14" width="16" height="2" rx="1" />
            </svg>
          </button>
          <h1 class="text-sm font-semibold tracking-tight">
            <a href="#/" class="hover:underline">Rust Design Patterns</a>
            <span class="mx-2 text-neutral-400">/</span>
            <span>{activeTrack() === 'gof' ? 'Gang of Four' : 'Rust-Idiomatic'}</span>
          </h1>
          <ThemeToggle />
        </header>

        <Show
          when={selected()}
          fallback={
            <div class="p-8 text-neutral-500">
              No patterns found for this track yet. Add one under{' '}
              <code>patterns/{activeTrack()}/</code>.
            </div>
          }
        >
          <PatternView
            pattern={selected()}
            track={activeTrack()}
            prev={neighbors().prev}
            next={neighbors().next}
          />
        </Show>
      </div>
    </div>
  );
}
