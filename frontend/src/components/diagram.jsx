import { onMount, createSignal } from 'solid-js';
import mermaid from 'mermaid';

let initialized = false;
function ensureInit() {
  if (initialized) return;
  mermaid.initialize({
    startOnLoad: false,
    theme: document.documentElement.classList.contains('dark') ? 'dark' : 'default',
    securityLevel: 'strict',
    fontFamily: 'ui-sans-serif, system-ui, sans-serif',
  });
  initialized = true;
}

let counter = 0;

export default function Diagram(props) {
  const [svg, setSvg] = createSignal('');
  const [error, setError] = createSignal('');

  onMount(async () => {
    ensureInit();
    const id = `mmd-${++counter}`;
    try {
      const { svg } = await mermaid.render(id, props.source);
      setSvg(svg);
    } catch (e) {
      setError(e?.message ?? String(e));
    }
  });

  return (
    <div class="mermaid-host my-6 flex justify-center overflow-x-auto">
      {error() ? (
        <pre class="text-rose-500 text-xs whitespace-pre-wrap">{error()}</pre>
      ) : (
        <div innerHTML={svg()} />
      )}
    </div>
  );
}
