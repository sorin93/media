<svelte:window {onpopstate} />

<Route />

<script>
  import { untrack, tick } from 'svelte';
  import Route from './Router/Route.svelte';
  import store from './lib/store.svelte';

  // Back navigation
  const onpopstate = async e => {
    if (e.state) {
      store.url = new URL(e.state.pathname, window.location.origin);
      await tick();
      window.scrollTo({ left: 0, top: e.state.scrollY || 0, behavior: 'instant' });
    }
  };

  $effect(() => {
    document?.documentElement?.setAttribute('data-theme', store.theme);
    const handleSystemChange = e => {
      if (!localStorage.getItem('theme')) store.theme = e.matches ? 'dark' : 'light';
    };
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', handleSystemChange);
    return () => mediaQuery.removeEventListener('change', handleSystemChange);
  });

  $effect(() => {
    if ('scrollRestoration' in window.history) window.history.scrollRestoration = 'manual';
    window.location;
    untrack(() => {
      if(!store.user) store.user = JSON.parse(localStorage.getItem('user'));
      if (!history.state) {
        window.history.replaceState({
          path: window.location.pathname + window.location.search,
          scrollY: 0,
        }, '', window.location.href);
      }
    });
  });
</script>