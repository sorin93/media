import store from '../lib/store.svelte';

export default node => {
  const click = e => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey || node.target === '_blank') return;
    const href = node.getAttribute('href');
    if (!href) return;
    try {
      const url = new URL(href, location.origin);
      url.tab = node.dataset.tab; // add new tab parameter to url
      if (url.origin !== location.origin) return;
			e.preventDefault();      
			store.url = url; // triggers parent $effect which calls init()
      if (url.pathname === location.pathname) return;
      history.replaceState({
        ...history.state,
        pathname: window.location.pathname,
        scrollY: scrollY,
      }, '', location.href);      
      history.pushState({
        pathname: url.pathname,
        scrollY: 0,
      }, '', url);
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