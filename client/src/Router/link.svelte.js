import store from '../lib/store.svelte';

export default node => {
  const click = e => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey || node.target === '_blank') return;
    const href = node.getAttribute('href');
    if (!href) return;
    try {
      const url = new URL(href, location.origin);
      if (url.origin !== location.origin) return;
			e.preventDefault();
			url.tab = node.dataset.tab; // transient tab state, consumed through store.url.tab
      store.url = url; // triggers update(); must happen before handling same-path tab changes
      if (url.pathname === location.pathname) {
          // same page: tab change only, reset vertical scroll
          if (node.dataset.tab) scrollTo({ top: 0, left: 0, behavior: 'instant' });
          return;
      }
      const replace = node.dataset.replace !== undefined;
      if (replace) {
        history.replaceState({
          pathname: url.pathname,
          scrollY: 0,
        }, '', url);
      } else {
        history.replaceState({
          ...history.state,
          pathname: window.location.pathname,
          scrollY: scrollY,
        }, '', location.href);

        history.pushState({
          ...history.state,
          pathname: url.pathname,
          scrollY: 0,
        }, '', url);
      }
			scrollTo({ top: 0, left: 0, behavior: 'instant' });
    } catch (_) {}
  };
  node.addEventListener('click', click);
  return {
    destroy() {
      node.removeEventListener('click', click);
    }
  }
};
