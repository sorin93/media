<svelte:window on:keydown={onkeydown} />

<!-- Player -->
{#snippet player()}
  <!-- Player -->
  {#key result.index}
    <img
      alt={result.media[result.index].caption || ''}
      class="image"
      draggable="false"
      {onwheel}
      src={getImageUrl(result.media[result.index].media_id, 'large')}
      style="background: url({getImageUrl(result.media[result.index].media_id, 'medium')}) center / cover no-repeat;"
    />
  {/key}

  <!-- Overlay -->
  <div class="overlay">
    {#if store.isMobile}
      <address class="align-center flex gap-4">
        <UserPic size={32} user={result.media[result.index].user} />
        <div class="white">{result.media[result.index].user?.name}</div>
      </address>
    {/if}
    {#if result.media[result.index].caption}
      <h1 class="reset">{result.media[result.index].caption}</h1>
    {/if}
  </div>

  <!-- Prev -->
  {#if result.index > 0}
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <a class="overlay-prev" href="/m/{result.media[result.index - 1].media_id}" use:link></a>
  {/if}

  <!-- Next -->
  {#if result.index < result.media.length - 1}
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <a class="overlay-next" href="/m/{result.media[result.index + 1].media_id}" use:link></a>
  {/if}

  <!-- Side menu -->
  {#if !store.isMobile}
    <div class="actions">

      <!-- Prev -->
      {#if result.index > 0}
        <a href="/m/{result.media[result.index - 1].media_id}" use:link>
          <Svg
            aria-hidden="true"
            class="pointer"
            fill="var(--fill)"
            name="keyboard_arrow_up"
            size="30px"
          />
        </a>
      {/if}

      <!-- Next -->
      {#if result.index < result.media.length - 1}
        <a href="/m/{result.media[result.index + 1].media_id}" use:link>
          <Svg
            aria-hidden="true"
            class="pointer"
            fill="var(--fill)"
            name="keyboard_arrow_down"
            size="30px"
          />
        </a>
      {/if}

      <!-- User -->
      <a
        href="/u/{result.media[result.index].user?.user_id}"
        data-tab="/u/{result.media[result.index].user?.user_id}:user_media"
        use:link
      >
        <UserPic size={32} user={result.media[result.index].user} />
      </a>

    </div>
  {/if}

{/snippet}

<!-- Loader -->
{#if status === 'loading'}
  <div class="loading"></div>
{:else}

  <!-- Message -->
  {#if status}<Message message={status} type="error" />{/if}

  <!-- Wrapper -->
  <div class="wrapper">
    {#if store.isMobile}

      <!-- Small -->
      <div class="center-small">

        {#if result?.media?.[result.index]}

          <!-- Media -->
          <main>
            <article class="media-small">
              {@render player()}
            </article>
          </main>

          <Tabs {tab} {tabs} />

          {#if result?.media?.length}
            <List {result} {tab} enlarged={result?.index >= 0} />
          {/if}

        {:else}

          <Header {tab} {result} />
          <!--{#if result?.user}-->
            <Tabs {tab} {tabs} />
          <!--{/if}-->

          {#if result?.media?.length}
            <List {result} {tab} enlarged={result?.index >= 0} />
          {/if}

        {/if}


      </div>

      <Navigation />

    {:else}

      <!-- Large -->
      <Navigation />

      <!-- Media -->
      {#if result?.media?.[result.index]}

        <!-- Center -->
        <main class="center-media">
          <article class="media-large">
            {@render player()}
          </article>
        </main>

        <!-- Right -->
        <aside>

          <User user={result.media[result.index]?.user} />
          <div class="sticky">
            <Tabs {tab} {tabs} />
          </div>

          {#if result?.media?.length}
            <List {result} {tab} enlarged={result?.index >= 0} />
          {/if}

        </aside>

      {:else}

        <!-- Center -->
        <div class="center-large">

          <Header {tab} {result} />
          <!--{#if result?.user}-->
            <Tabs {tab} {tabs} />
          <!--{/if}-->

          {#if result?.media?.length}
            <List {result} {tab} enlarged={result?.index >= 0} />
          {/if}

        </div>

      {/if}

    {/if}
  </div>

{/if}

<style>
  .wrapper {
    --gap: 0;
    --nav-width: 0;
    box-sizing: border-box;
    display: flex;
    gap: var(--gap);
    margin-left: var(--nav-width);
    overflow: hidden;
  }
  @media (min-width: 1024px) {
    .wrapper {
      --gap: 16px;
      --nav-width: 66px; /* 50 + 16 */
    }
  }
  @media (min-width: 1200px) {
    .wrapper {
      --nav-width: 216px; /* 200 + 16 */
    }
  }

  .center-small,
  .center-large,
  .center-media {
    box-sizing: border-box;
    display: flex;
    flex: 1;
    gap: 16px;
    min-width: 0;
    overflow: hidden;
  }

  .center-small {
    flex-direction: column;
    margin: 0 auto;
    max-width: 768px;
  }
  .center-large {
    flex-direction: column;
    margin: 0 auto;
    padding: 0 32px;
  }

  .center-media {
    flex: 1 1 auto;
    align-items: center;
    justify-content: center;
  }

  aside {
    background-color: var(--bg-secondary);
    border-left: 1px solid var(--border);
    box-sizing: border-box;
    container-type: inline-size;
    display: flex;
    flex: 1 1 300px;
    flex-direction: column;
    gap: 16px;
    height: 100dvh;
    min-width: 300px;
    max-width: 500px;
    overflow-y: auto;
  }

  .media-small,
  .media-large {
    aspect-ratio: 9 / 16;
    box-sizing: border-box;
    _overflow: hidden;
    position: relative;
  }

  .media-small {
    max-height: 90dvh;
    width: 100%;
  }

  .media-large {
    border-radius: 16px;
    flex: 0 0 auto;
    height: auto;
    width: min(100%, calc((100dvh - 32px) * 9 / 16));
  }

  .image {
    display: block;
    height: 100%;
    object-fit: cover;
    object-position: center;
    width: 100%;
  }

  .overlay {
    background: linear-gradient(to top, #0000000f 0%, transparent 100%);
    bottom: 0;
    color: white;
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 40%;
    justify-content: flex-end;
    left: 0;
    overflow: hidden;
    padding: 4px;
    position: absolute;
    right: 0;
  }

  .overlay-next,
  .overlay-prev {
    height: 33.33%;
    left: 0;
    position: absolute;
    right: 0;
  }

  .overlay-prev { top: 0 }
  .overlay-next { bottom: 0 }

  .sticky {
    background: inherit;
    display: flex;
    flex-direction: column;
    gap: 16px;
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .actions {
    --fill: white;
    bottom: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    position: absolute;
    right: 16px;
  }
  @media (min-width: 1600px) {
    .actions {
      --fill: var(--text-primary);
      bottom: 0;
      left: calc(100% + 16px);
      right: unset;
    }
  }
</style>

<script>
  import { untrack } from 'svelte';
  import Header from './Header.svelte';
  import List from './List.svelte';
  import Message from '../Misc/Message.svelte';
  import Navigation from './Navigation.svelte';
  import Svg from '../Svg/Svg.svelte';
  import Tabs from './Tabs.svelte';
  import User from './User.svelte';
  import UserPic from '../User/UserPic.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';
  import link from '../Router/link.svelte';
  import redirect from '../Router/redirect.svelte';
  import store from '../lib/store.svelte';

  let result = $state.raw({});
  let status = $state.raw();
  let timestamp = $state.raw();
  let tab = $state.raw();
  let tabs = $state.raw([]);

  // step
  const step = dir => {
    if (Date.now() - timestamp < 250) return;
    timestamp = Date.now();
    redirect(`/m/${result.media[result.index + dir].media_id}`, { list: true });
  };

  // onkeydown
  const onkeydown = e => {
    if (result?.index > 0 && (e.key === 'ArrowUp' || e.key === 'PageUp')) {
      e.preventDefault();
      step(-1);
    } else if (result?.index < result?.media?.length - 1 && (e.key === 'ArrowDown' || e.key === 'PageDown')) {
      e.preventDefault();
      step(1);
    }
  };

  // onwheel
  const onwheel = e => {
    e.preventDefault();
    if (result?.index > 0 && e.deltaY < -50) step(-1);
    if (result?.index < result?.media?.length - 1 && e.deltaY > 50) step(1);
  };

  // Get data
  /*
    id is always path[2] (user_id or media_id)


  */

  const getData = async more => {
    if (status === 'loading' || !tab) return;
    // Cache
    const cache = store.getCache(tab);
    const offset = cache?.media?.length ?? 0;
    const path = store.url?.pathname?.split('/');
    // todo: subject is deprecated
    let subject = window.history.state?.subject;
    if (offset && !more) {
      try {
        if (path[1] === 'm' && !subject) subject = await fetchApi('get', `/media/${path[2]}`);
        if (path[1] === 'u' && !subject) subject = await fetchApi('get', `/users/${path[2]}`);
      } catch (e) {
        status = e.message;
        return;
      }
      result = { ...cache, subject };

      // todo: gather subject{} from state or expand=true
      const index = result?.media?.findIndex(m => m.media_id === path[2]);
      if (index >= 0) result.index = index;

      return;
    }
    // Api
    try {
      status = 'loading';
      let res;
      const payload = { page: more ? Math.floor(offset / 100) + 1 : 1 };
      if (tab === '/explore:') {
        res = await fetchApi('get', '/explore', payload);
      } else if (tab === '/following:') {
        if (!store.user?.user_id) {
          redirect('/explore', { replace: true });
          return;
        }
        res = await fetchApi('get', '/following', payload);
      } else if (tab.slice(0, 3) === '/s/') {
        payload.query = decodeURIComponent(tab.slice(3).slice(0, -1));
        res = await fetchApi('get', '/search', payload);
      } else if (tab.slice(0, 3) === '/u/') {
        res = await fetchApi('get', tab.replace('/u/', '/users/').replace(':user_', '/'), payload);
      } else if (tab.slice(0, 3) === '/m/') {
        if (tab.slice(-11) === ':user_media') {
          res = await fetchApi('get', `/media/${tab.split('/')[2].split(':')[0]}/user`, payload);
        } else {
          payload.expand = true; // todo
          res = await fetchApi('get', tab.replace('/m/', '/media/').replace(':media_', '/'), payload);
        }
      } else throw 'Error';
      if (more) {
        // todo
      } else {
        result = { ...store.setCache(tab, res, more) };
      }
      console.log(result);


      const target = store.url?.pathname?.slice(0, 3) === '/m/' ? store.url?.pathname.slice(3) : undefined;
      const index = result?.media?.findIndex(m => m.media_id === target); // todo: deprecate
      if (index >= 0) result.index = index;  // todo: deprecate

      if (target) {
        // todo: if target object is in the current list get it from there, otherwise fetch it from the cache or the api
      }

      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // init2
  /*
  const init2 = newTab => {
    let [, type, id] = store.url?.pathname?.split('/') ?? [];
    if (type === 'search') id = decodeURIComponent(id);
    tabs = type === 'explore' || type === 'following' ? [type] :
      type === 'search' ? [`search:${id}`] :
      type === 'u' ? [`user_media:${id}`, `user_comments:${id}`, `user_likes:${id}`] :
      type === 'm' ? [`media_suggestions:${id}`, `media_comments:${id}`, `user_media:${id}`] : [];
    if (!tabs.length) return;
    const trail = window.history.state?.trail ?? [];
    tab = tabs.includes(newTab) ? newTab : tabs.includes(trail.at(-1)) ? trail.at(-1) : tabs[0];
    window.history.replaceState({ ...window.history.state, trail: [...trail.slice(0, -1), tab] }, '', window.location.href);
    getData();
  };
  */

  // never do <a href="..." data-tab={tab}>, which basically says set new tab to current tab, simply pass <a href="...">
  // only pass <a href="..." data-tab={new_tab}> when i change the active tab

  // init
  const init = () => {
    status = undefined;
    if (store.url.tab) tab = store.url.tab;
    const [, type, id] = location.pathname?.split('/') ?? [];
    if (type === 'explore' || type === 'following') tabs = [`/${type}:`];
    if (type === 's') tabs = [`/s/${id}:`];
    if (type === 'u') tabs = [`/u/${id}:user_media`, `/u/${id}:user_comments`, `/u/${id}:user_likes`];
    if (type === 'm') {
      tabs = [`/m/${id}:media_suggestions`, `/m/${id}:media_comments`, `/m/${id}:user_media`];
      if (tab && !tabs.includes(tab) && tab.slice(0, 3) !== '/m/') tabs.unshift(tab);
    }
    if (!tab) tab = tabs[0];
    console.log(tab, tabs);
    getData();

    /*
    // associate the current pathname with the contextual object i'm coming from (media, user, explore, etc.), as a tab
    if (location.pathname !== tab?.split(':')?.[0]) store.tabs.set(location.pathname, tab);

    // compose current[] tabs based on new pathname
    let [, type, id] = location.pathname?.split('/') ?? [];
    const current = type === 'explore' || type === 'following' ? [`/${type}:`] :
      type === 's' ? [`/s/${id}:`] :
      type === 'u' ? [`/u/${id}:user_media`, `/u/${id}:user_comments`, `/u/${id}:user_likes`] :
      type === 'm' ? [`/m/${id}:media_suggestions`, `/m/${id}:media_comments`, `/m/${id}:user_media`] : [];
    if (!current.length) return;

    // compose parent tab
    let parent = store.tabs.get(location.pathname);
    let grandparent = store.tabs.get(parent?.split(':')?.[0]);
    if (parent?.split(':')?.[0] === current?.[0]?.split(':')?.[0]) {
      // todo: specifically for type
      parent = grandparent;
      grandparent = undefined;
    }

    // compose grandparent tab
    if (grandparent?.split(':')?.[0] === parent?.[0]?.split(':')?.[0]) {
      // todo: specifically for type
      grandparent = undefined;
    }

    tabs = [grandparent, parent, current];

    // new current tab
    if (store.url.tab) tab = store.url.tab; // new set tab
    if (!tab || (!current.includes(tab) && tab !== parent && tab !== grandparent)) tab = current[0]; // default if invalid tab

    if (tabs[0]) console.log('grandparent', tabs[0]);
    if (tabs[1]) console.log('parent', tabs[1]);
    console.log('current');
    if (tabs[2][0]) console.log('   ', tabs[2][0]);
    if (tabs[2][1]) console.log('   ', tabs[2][1]);
    if (tabs[2][2]) console.log('   ', tabs[2][2]);
    console.log('tab', tab);
    */
  };

  // Track store.url
  $effect(() => {
    store.url;
    untrack(init);
  });
</script>