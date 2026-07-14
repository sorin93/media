<!-- Explore -->
{#if tab === 'explore'}
  <h1 class="reset">Explore</h1>

<!-- Following -->
{:else if tab === 'following'}
  <h1 class="reset">Following</h1>

<!-- Search -->
{:else if tab === 'search'}
  <h1 class="reset">{decodeURIComponent(store.url?.type.slice(3)) || 'Search'}</h1>

<!-- User -->
{:else if tab?.slice(0, 4) === 'user' && result?.user}
  <User user={result?.user ?? result?.media?.user} />

{/if}

<script>
  import User from './User.svelte';
  import store from '../lib/store.svelte';

  const { tab, result } = $props();
</script>