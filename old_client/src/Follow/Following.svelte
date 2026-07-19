<!-- Following -->
{#if following}
  <hr>
  <br>
  <div class="flex gap-16">
    {#each following as user}
      <Avatar user={user} />
    {/each}
  </div>
{/if}

<script>
  import { untrack } from 'svelte';
  import Avatar from '../User/Avatar.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  let
    following = $state(store.cache.get('/following')?.following || []),
    status = $state();

  // Get following
  const getFollowing = async () => {
    if (status === 'loading') return;
    const created_at = store.cache?.get('/following')?.created_at || 0;
    if (Date.now() - created_at < 3600000) return;
    status = 'loading';
    try {
      const res = await fetchApi('get', '/following');
      store.cache.set('/following', { created_at: Date.now(), ...res });
      following = res.following;
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // Mount
  $effect(() => untrack(() => getFollowing()));
</script>