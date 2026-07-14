<div class="layout-margin">

  <!-- Menu -->
  <div class="align-center flex gap-16">
    <h1 class="bold flex-grow green italic">Media</h1>
    <Svg
      aria-hidden="true"
      class="pointer"
      fill="gray"
      name="add"
      onclick={() => redirect('/media/new')}
      size="30px"
    />
    <Svg
      aria-hidden="true"
      class="pointer"
      fill="gray"
      name="search"
      onclick={() => showSearch = !showSearch}
      size="30px"
    />
    <Svg
      aria-hidden="true"
      class="pointer"
      fill="gray"
      name="menu"
      onclick={() => {}}
      size="30px"
    />
  </div>

  <!-- Seach -->
  {#if showSearch}
    <form {onsubmit}>
      <Input
        focus
        label="Search"
        maxlength="100"
        minlength="3"
        required
        style="max-width: 540px"
        tabindex={1}
        bind:value={search}
      />
    </form>
    <br><br>
  {/if}

  <!-- Explore -->
  {#if pathname === 'explore'}
    <h1>Explore</h1>

  <!-- Following -->
  {:else if pathname === 'following'}
    <h1>Following</h1>

  <!-- Text -->
  {:else if pathname === 's'}
    <h1>{decodeURIComponent(store.url?.pathname.slice(3)) || 'Search'}</h1>

  <!-- User -->
  {:else if pathname === 'u' && result?.user}
    <div class="align-center flex gap-8">
      <UserPic size={48} user={result.user} />
      <h1>{result.user?.name}</h1>
    </div>

  {/if}

</div>

<style>
  h1 { margin: 0 }
</style>

<script>
  import { untrack } from 'svelte';
  import Input from '../Misc/Input.svelte';
  import Svg from '../Svg/Svg.svelte';
  import UserPic from '../User/UserPic.svelte';
  import redirect from '../Router/redirect.svelte';
  import store from '../lib/store.svelte';

  const { pathname = undefined, result = undefined } = $props();

  let
    search = $state(''),
    showSearch = $state(false);

  const onsubmit = e => {
    e?.preventDefault();
    search = search.trim().toLocaleLowerCase();
    if (search?.length < 3) return;
    document.activeElement.blur();
    redirect(`/t/${search}`);
  };

  // Track store.url.pathname
  $effect(() => {
    store.url?.pathname;
    untrack(() => {
      // search = store.url?.pathname?.slice(0, 3) === '/t/' ? decodeURIComponent(store.url?.pathname.slice(3) || '') : '';
      search = '';
      showSearch = false;
    });
  });
</script>