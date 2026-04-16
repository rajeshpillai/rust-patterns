import { createSignal } from 'solid-js';

const STORAGE_KEY = 'patterns-rust-theme';

const [theme, setTheme] = createSignal(
  typeof localStorage !== 'undefined'
    ? localStorage.getItem(STORAGE_KEY) ?? 'dark'
    : 'dark',
);

export function initTheme() {
  applyTheme(theme());
}

export function useTheme() {
  return [theme, toggleTheme];
}

function toggleTheme() {
  const next = theme() === 'dark' ? 'light' : 'dark';
  setTheme(next);
  localStorage.setItem(STORAGE_KEY, next);
  applyTheme(next);
}

function applyTheme(t) {
  const root = document.documentElement;
  if (t === 'dark') root.classList.add('dark');
  else root.classList.remove('dark');
}
