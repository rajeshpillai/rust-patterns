/* @refresh reload */
import { render } from 'solid-js/web';
import { HashRouter, Route } from '@solidjs/router';
import './index.css';

import Landing from './pages/landing.jsx';
import PatternBrowser from './pages/pattern-browser.jsx';
import { initTheme } from './theme.js';

initTheme();

const root = document.getElementById('root');

render(
  () => (
    <HashRouter>
      <Route path="/" component={Landing} />
      <Route path="/patterns" component={PatternBrowser} />
      <Route path="/patterns/:track" component={PatternBrowser} />
      <Route path="/patterns/:track/:id" component={PatternBrowser} />
    </HashRouter>
  ),
  root,
);
