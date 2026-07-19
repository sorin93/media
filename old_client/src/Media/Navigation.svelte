<svelte:window bind:innerWidth={width} />

<nav>
  <a class="reset" href="/explore" data-tab="/explore:" use:link>{#if compact}E{:else}Explore{/if}</a>
  {#if store.user?.user_id}
    <a href="/following" data-tab="/following:" use:link>{#if compact}F{:else}Following{/if}</a>
    <a href="/u/{store.user.user_id}" data-tab="/u/{store.user.user_id}:user_media" use:link>{#if compact}M{:else}My Media{/if}</a>
    <a href="." onclick={e => { e.preventDefault(); store.toggleTheme(); }}>{#if compact}T{:else}Theme{/if}</a>
  {/if}
</nav>

<style>
  nav {
    --color: #00000044;
    --nav-width: 100%;
    background-color: var(--color);
    _border-top: 1px solid var(--border);
    bottom: 0;
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    gap: 16px;
    justify-content: space-between;
    margin: 0 auto;
    padding: 16px;
    position: fixed;
    width: var(--nav-width);
  }
  @media (min-width: 1024px) {
    nav {
      --color: var(--bg-secondary);
      --nav-width: 50px;
      border: 1px solid var(--border);
      bottom: unset;
      flex-direction: column;
      left: 0;
      margin: 16px;
      top: 0;
    }
  }
  @media (min-width: 1200px) {
    nav {
      --nav-width: 200px;
    }
  }
</style>

<script>
  import link from '../Router/link.svelte';
  import store from '../lib/store.svelte';

  let width = $state(0);
  const compact = $derived(width < 1200);
</script>