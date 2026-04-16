import { onMount, createEffect } from 'solid-js';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js/lib/core';
import rust from 'highlight.js/lib/languages/rust';
import mermaid from 'mermaid';

hljs.registerLanguage('rust', rust);

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: false,
  highlight: (source, lang) => {
    if (lang === 'mermaid') {
      // Defer to post-render hydration — stash the raw source.
      const encoded = encodeURIComponent(source);
      return `<div class="mermaid-fence" data-src="${encoded}"></div>`;
    }
    if (lang === 'rust' && hljs.getLanguage('rust')) {
      return `<pre class="hljs language-rust"><code>${
        hljs.highlight(source, { language: 'rust' }).value
      }</code></pre>`;
    }
    return `<pre><code>${md.utils.escapeHtml(source)}</code></pre>`;
  },
});

// markdown-it wraps `highlight`'s output in another <pre><code>...</code></pre>
// when the caller returns plain text. Returning HTML with the opening tag
// bypasses that wrap. For `mermaid` we want no <pre> wrap at all, so the
// highlight return above deliberately does not start with <pre><code>.
// To suppress the default wrap for our mermaid case, override fence renderer:
const defaultFence = md.renderer.rules.fence;
md.renderer.rules.fence = function (tokens, idx, options, env, self) {
  const token = tokens[idx];
  if (token.info.trim() === 'mermaid') {
    const encoded = encodeURIComponent(token.content);
    return `<div class="mermaid-fence" data-src="${encoded}"></div>\n`;
  }
  return defaultFence(tokens, idx, options, env, self);
};

let mermaidReady = false;
function initMermaid() {
  if (mermaidReady) return;
  mermaid.initialize({
    startOnLoad: false,
    theme: document.documentElement.classList.contains('dark') ? 'dark' : 'default',
    securityLevel: 'strict',
    fontFamily: 'ui-sans-serif, system-ui, sans-serif',
  });
  mermaidReady = true;
}

let mmdCounter = 0;

async function hydrateMermaid(container) {
  const nodes = container.querySelectorAll('.mermaid-fence');
  for (const node of nodes) {
    if (node.dataset.rendered === 'true') continue;
    const src = decodeURIComponent(node.dataset.src ?? '');
    const id = `mmd-${++mmdCounter}`;
    try {
      const { svg } = await mermaid.render(id, src);
      node.innerHTML = svg;
      node.dataset.rendered = 'true';
    } catch (e) {
      node.innerHTML = `<pre class="text-rose-500 text-xs whitespace-pre-wrap">${
        String(e?.message ?? e)
      }</pre>`;
    }
  }
}

export default function Markdown(props) {
  let ref;
  const html = () => md.render(props.source ?? '');

  onMount(initMermaid);

  createEffect(() => {
    // Re-run when source changes.
    void props.source;
    queueMicrotask(() => {
      if (ref) hydrateMermaid(ref);
    });
  });

  return <div ref={ref} class="prose-pattern max-w-none" innerHTML={html()} />;
}
