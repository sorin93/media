{#if store.user?.user_id && store.user?.user_id !== user_id && store.following}
  {#if isFollowing}
    <button onclick={unfollow}>Unfollow</button>
  {:else}
    <button onclick={follow}>Follow</button>
  {/if}
{/if}

<script>
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  // Follow
  const follow = async () => {
    if (status === 'loading') return;
    status = 'loading';
    try {
      const res = await fetchApi('post', `/following/${user_id}`);
      store.user.following_count ++;
      store.user = store.user;
      store.following = [ res.user, ...(store.following ?? [])];
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // Unfollow
  const unfollow = async () => {
    if (status === 'loading') return;
    status = 'loading';
    try {
      const res = await fetchApi('delete', `/following/${user_id}`);
      store.user.following_count --;
      store.following = store.following.filter(u => u.user_id !== user_id);
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  const { user_id } = $props();

  let
    isFollowing = $derived(store.following.some(u => u.user_id === user_id)),
    status = $state();
</script>