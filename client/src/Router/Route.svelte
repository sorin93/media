<Component />

<script>
  import NotFound from './NotFound.svelte';
  import Unauthorized from './Unauthorized.svelte';
  import { untrack } from 'svelte';
  import store from '../lib/store.svelte';
  import redirect from './redirect.svelte';
  import routes from './routes';

  let Component = $state();

  // Track store.url.pathname, store.user
  $effect(() => {
    store.url;
    store.user;
    untrack(() => {
      const route = routes.find(route =>
        (typeof route[5] === 'string' && route[5] === store.url?.pathname) ||
        (typeof route[5] === 'object' && route[5].test(store.url?.pathname))
      );
      if (route) {
        if (route[3] === null || route[3] === Boolean(store.user?.user_id)) {
          if (Component !== route?.[2]) {
            Component = route[2];
            window.scrollTo({ left: 0, top: 0, behavior: 'instant' });
          }
        } else if (route[3] === false && store.user?.user_id) {
          redirect('/', { replace: true });
        } else {
          Component = Unauthorized;
        }
      } else {
        Component = NotFound;
      }
    });
  });
</script>