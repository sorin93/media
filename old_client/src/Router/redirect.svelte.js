// todo: fix redirect same as link.svelte.js

import store from '../lib/store.svelte';

export default (url, options = {}) => {
  const { replace = false, context = undefined } = options;
  try {
    const
      targetUrl = new URL(url, window.location.origin),
      currentPath = window.location.pathname + window.location.search,
      nextPath = targetUrl.pathname + targetUrl.search;
    if (currentPath === nextPath && targetUrl.hash === window.location.hash) return;

    // Context
    if (nextPath !== context) context ? store.tabs.set(nextPath, context) : store.tabs.delete(nextPath);

    if (replace) {
      // Replace current state
      window.history.replaceState({
        ...window.history.state,
        pathname: nextPath,
        scrollY: 0,
      }, '', targetUrl.href);
    } else {
      // Save current state
      window.history.replaceState({
        ...window.history.state,
        pathname: currentPath,
        scrollY: window.scrollY,
      }, '', window.location.href);
      // Push new state
      window.history.pushState({
        path: nextPath,
        scrollY: 0,
      }, '', targetUrl.href);
    }

    window.scrollTo({ left: 0, top: 0, behavior: 'instant' });
    store.url = targetUrl;
  } catch (_) {}
};