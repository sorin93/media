import store from '../lib/store.svelte';

export default node => {
  const click = e => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    if (node.target === '_blank') return;
    const href = node.getAttribute('href');
    if (!href) return;

    try {
      const targetUrl = new URL(href, window.location.origin);
      if (targetUrl.origin !== window.location.origin) return;
      const
        currentPath = window.location.pathname + window.location.search,
        nextPath = targetUrl.pathname + targetUrl.search;
      if (nextPath === currentPath) {
        if (targetUrl.hash === window.location.hash) e.preventDefault();
        return;
      }
      e.preventDefault();

      // Context
      const context = node.dataset.context;
      if (nextPath !== context) context ? store.tabs.set(nextPath, context) : store.tabs.delete(nextPath);

      // Save current state
      window.history.replaceState({
        ...window.history.state,
        path: currentPath,
        scrollY: window.scrollY,
      }, '', window.location.href);
      // Push new state
      window.history.pushState({
        path: nextPath,
        scrollY: 0,
      }, '', targetUrl.href);
      window.scrollTo({ left: 0, top: 0, behavior: 'instant' });
      store.url = targetUrl;
    } catch (_) {}
  };

  node.addEventListener('click', click);

  return {
    destroy() {
      node.removeEventListener('click', click);
    }
  };
};