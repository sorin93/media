<svelte:window on:keydown={onkeydown} />

<!-- Player -->
{#snippet player()}
  {#if store.enlarged?.media}

    <!-- Player -->
    {#key store.enlarged.media}
      <div class="image-container">
        <img
          alt={store.enlarged.media.caption || ''}
          class="image"
          draggable="false"
          {onwheel}
          src={getImageUrl(store.enlarged.media.media_id, 'large')}
          style="background: url({getImageUrl(store.enlarged.media.media_id, 'medium')}) center / cover no-repeat;"
        />
      </div>
    {/key}

    <!-- Overlay -->
    <div class="overlay">

      <!-- User pic and name -->
      {#if store.isMobile}
        <address class="align-center flex gap-4">
          <UserPic size={32} user={store.enlarged.media.user} />
          <div class="white">{store.enlarged.media.user?.name}</div>
        </address>
      {/if}

      <!-- Caption -->
      {#if store.enlarged.media.caption}
        <h1 class="reset">{store.enlarged.media.caption}</h1>
      {/if}
    </div>


    <!-- Prev -->
    {#if prev()}
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <a class="overlay-prev" href="." onclick={event => enlargeMedia(...prev(), event)}></a>
    {/if}

    <!-- Next -->
    {#if next()}
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <a class="overlay-next" href="." onclick={event => enlargeMedia(...next(), event)}></a>
    {/if}

    <!-- Close -->
    {#if store.isMobile}
      <a class="close" href="." onclick={closeMedia}>
        <Svg
          aria-hidden="true"
          class="pointer"
          fill="var(--fill)"
          name="close"
          size="30px"
        />
      </a>
    {/if}

    <!-- Side menu -->
    <div class="actions">

      <!-- Prev -->
      {#if prev()}
        <a href="." onclick={event => enlargeMedia(...prev(), event)}>
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
      {#if next()}
        <a href="." onclick={event => enlargeMedia(...next(), event)}>
          <Svg
            aria-hidden="true"
            class="pointer"
            fill="var(--fill)"
            name="keyboard_arrow_down"
            size="30px"
          />
        </a>
      {/if}


      <!-- Similar media -->
      <a href="." onclick={event => newContext(`/m/${store.enlarged?.media?.media_id}:media_suggestions`, event)}>
        <Svg
          aria-hidden="true"
          class="pointer"
          fill="var(--fill)"
          name="grid_view"
          size="30px"
        />
      </a>

      <!-- From user -->
      <a href="." onclick={event => newContext(`/u/${store.enlarged?.media?.user?.user_id}:user_media`, event)}>
        <UserPic size={32} user={store.enlarged.media.user} />
      </a>

    </div>

  {/if}

{/snippet}

<!-- Message -->
{#if false && status && status !== 'loading'}<Message message={status} type="error" />{/if}

<!-- Wrapper -->
<div class="wrapper">
  {#if store.isMobile}

    <!-- Small -->
    <div class="center-small">
      <Feed {closeContext} {enlargeMedia} {newContext} />
    </div>

    <!-- Media -->
    {#if store.enlarged}
      <main>
        <article class="media-small">
          {@render player()}
        </article>
      </main>
    {/if}

    {#if !store.enlarged}
      <Navigation />
    {/if}

  {:else}

    <!-- Large -->
    <Navigation />

    <!-- Media -->
    {#if store.enlarged?.media}

      <!-- Center -->
      <main class="center-media">
        <article class="media-large">
          {@render player()}
        </article>
      </main>

      <!-- Right -->
      <aside>
        <Feed {closeContext} {enlargeMedia} {newContext} />
      </aside>

    {:else}

      <!-- Center -->
      <div class="center-large">
        <Feed {closeContext} {enlargeMedia} {newContext} />
      </div>

    {/if}

  {/if}
</div>

<!-- Loader -->
{#if false && status === 'loading'}
  <div class="loading"></div>
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

  .media-large,
  .image-container {
    aspect-ratio: 9 / 16;
    box-sizing: border-box;
    position: relative;
  }

  .media-small {
    align-items: center;
    background-color: var(--bg-primary);
    display: flex;
    inset:0;
    justify-content: center;
    position: fixed;
  }

  .media-large {
    flex: 0 0 auto;
    height: auto;
    _overflow: hidden;
    width: min(100%, calc((100dvh - 32px) * 9 / 16));
  }

  .image-container {
    border-radius: 16px;
    height: 100%;
    width: 100%;
  }

  .image {
    display: block;
    height: 100%;
    object-fit: contain;
    width: 100%;
  }

  .media-small .image-container {
    align-items: center;
    aspect-ratio: 9 / 16;
    display: flex;
    height: auto;
    justify-content: center;
    width: min(100%, calc(100dvh * 9 / 16));
  }

  .media-small .image {
    height: 100%;
    object-fit: contain;
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

  .close {
    --fill: white;
    background: #00000066;
    border-radius: 64px;
    left: 16px;
    padding: 8px;
    position: absolute;
    top: 16px;
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

  @media (min-width: 1024px) {
    .wrapper {
      --gap: 16px;
      --nav-width: 66px; /* 50 + 16 */
    }
    .image-container {
      overflow: hidden;
    }
  }

  @media (min-width: 1200px) {
    .wrapper {
      --nav-width: 216px; /* 200 + 16 */
    }
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
  import { untrack, settled } from 'svelte';
  import Feed from './Feed.svelte';
  import Header from './Header.svelte';
  import Message from '../Misc/Message.svelte';
  import Navigation from './Navigation.svelte';
  import Svg from '../Svg/Svg.svelte';
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
  let tab = $state.raw(); // Currently selected tab.
  let tabs = $state.raw([]); // Available tabs for the current route and context.
  let context = $state.raw(); // Active media context; set by update() for non-/m/ routes.

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

  const getData = async more => {
    if (status === 'loading' || !tab) return;
    // Cache
    // const cache = store.getCache(tab);
    // const offset = cache?.media?.length ?? 0;
    const offset = 0;
    const path = store.url?.pathname?.split('/');
    // todo: subject is deprecated
    /*
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
    */
    // Api
    try {
      status = 'loading';
      let res;
      const payload = more ? { page: Math.floor(offset / 100) + 1 } : { expand: true };
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
      /*
      if (more) {
        // todo
      } else {
        result = { ...store.setCache(tab, res, more) };
      }
      console.log(result);
      */

      const subject = res.subject;
      if (store.enlarged) {
        const { contextIndex, mediaIndex } = store.enlarged;
        // const exclude = new Set(store.feed[contextIndex].media.map(item => item.media_id));
        const exclude = new Set();
        const media = res.media.filter(it => !exclude.has(it.media_id));
        if (media.length) {
          store.feed[contextIndex].media = store.feed[contextIndex].media.slice(0, mediaIndex + 1);
          store.feed = [ ...store.feed.slice(0, contextIndex + 1), { tab, subject, media }];
        }
      } else {
        const media = res.media;
        store.feed = [{ tab, subject, media }];
      }

      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // update
  const update = () => {
    status = undefined;
    if (store.url.tab) tab = store.url.tab;
    const [, type, id] = location.pathname?.split('/') ?? [];
    if (type === 'explore' || type === 'following') {
      context = `/${type}:`;
      tabs = [`/${type}:`];
    } else if (type === 's') {
      context = `/s/${id}:`;
      tabs = [`/s/${id}:`];
    } else if (type === 'u') {
      context = `/u/${id}:user_media`;
      tabs = [
        `/u/${id}:user_media`,
        `/u/${id}:user_comments`,
        `/u/${id}:user_likes`
      ];
    } else if (type === 'm') {
      tabs = [
        `/m/${id}:media_suggestions`,
        `/m/${id}:media_comments`
      ];
      // if (context?.startsWith('/u/')) tabs.push(`/m/${id}:user_media`);
      if (context && !tabs.includes(context)) tabs.push(context);
    }
    if (!tabs.includes(tab)) tab = tabs[0];
    getData();
  };

  const enlargeMedia = (contextIndex, mediaIndex, event) => {
    event?.preventDefault();
    const element = document.getElementById(`${contextIndex}:${mediaIndex}`);
    if (!element) return;
    const rect = element.getBoundingClientRect();
    const scrollY = rect?.bottom;
    const media = store.feed[contextIndex]?.media?.[mediaIndex];
    store.enlarged = { contextIndex, mediaIndex, scrollY, media };
  };

  const closeMedia = e => {
    e?.preventDefault();
    e?.stopPropagation();
    store.enlarged = undefined;
  };

  const closeContext = async (contextIndex, event) => {
    event?.preventDefault();
    tab = store.feed[contextIndex]?.tab;
    if (!contextIndex || !tab) return;
    await getData(true);
    // todo: scrollY
    await settled();
    // todo: scrollY
  };

  const newContext = async (context, event) => {
    event?.preventDefault();
    // Context didn't change
    if (context === store.feed[store.enlarged?.contextIndex]?.tab) {
      store.enlarged = undefined;
      return;
    }
    tab = context;
    await getData();
    const scrollY = store.enlarged?.scrollY;
    if (store.isMobile) store.enlarged = undefined;
    await settled();
    const scroller = document.querySelector('aside') ?? window;
    if (scrollY) scroller.scrollBy({ top: scrollY, behavior: 'smooth' });
  };

  const prev = () => {
    const { contextIndex, mediaIndex } = store.enlarged;
    if (mediaIndex > 0) return [contextIndex, mediaIndex - 1];
    for (let i = contextIndex - 1; i >= 0; i--) {
      if (store.feed[i].media.length > 0) return [i, store.feed[i].media.length - 1];
    }
  };

  const next = () => {
    const { contextIndex, mediaIndex } = store.enlarged;
    if (mediaIndex + 1 < store.feed[contextIndex].media.length) return [contextIndex, mediaIndex + 1];
    for (let i = contextIndex + 1; i < store.feed.length; i++) {
      if (store.feed[i].media.length > 0) return [i, 0];
    }
  };

  // Track store.url
  $effect(() => {
    store.url;
    untrack(update);
  });
</script>
