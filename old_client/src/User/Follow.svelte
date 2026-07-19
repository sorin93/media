<script>
  import { untrack } from 'svelte';
  import store from '../lib/store.svelte';

  const { user_id } = $props();

  let user = $state();

  // Track user_id
  $effect(() => {
    user_id;
    untrack(() => {
      const user_id = store.history[0]?.type === 'user' ? store.history[0]?.value : medium?.user_id;
      user = store.history[0]?.type === 'following' ?
        store.user :
        users?.find(u => u.user_id === user_id);
    });
  });
</script>