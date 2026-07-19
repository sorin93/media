<br><br>
<div class="layout-margin">
  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}
    {#if status}<Message message={status} type="error" />{/if}
  {/if}
</div>

<script>
  import { untrack } from 'svelte';
  import Message from '../Misc/Message.svelte';
  import redirect from '../Router/redirect.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  let status = $state();

  const logout = async () => {
    status = 'loading'
    try {
      const res = await fetchApi('post', '/auth/logout');
      localStorage.clear();
      store.cache = new Map();
      store.tabs = new Map();
      store.theme = 'system';
      store.user = undefined;
      redirect('/login', { replace: true });
    } catch (e) {
      status = e.message;
    }
  };

  // Mount
  $effect(() => untrack(() => logout()));
</script>