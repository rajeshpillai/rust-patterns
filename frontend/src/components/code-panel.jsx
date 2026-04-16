import hljs from 'highlight.js/lib/core';
import rust from 'highlight.js/lib/languages/rust';

hljs.registerLanguage('rust', rust);

export default function CodePanel(props) {
  const highlighted = () => hljs.highlight(props.source ?? '', { language: 'rust' }).value;
  const playgroundUrl = () => props.playground ?? null;

  return (
    <div class="rounded-lg border border-neutral-800 overflow-hidden my-4">
      <div class="flex items-center justify-between bg-neutral-900 text-neutral-200 px-3 py-1.5 text-xs">
        <span class="font-mono">{props.filename}</span>
        {playgroundUrl() && (
          <a
            href={playgroundUrl()}
            target="_blank"
            rel="noreferrer"
            class="px-2 py-1 rounded bg-amber-600 hover:bg-amber-500 text-white font-medium"
          >
            Open in Playground ↗
          </a>
        )}
      </div>
      <pre class="m-0 p-4 overflow-x-auto bg-neutral-950 text-sm">
        <code class="hljs language-rust" innerHTML={highlighted()} />
      </pre>
    </div>
  );
}
