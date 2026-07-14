<!-- Loader -->
{#if status === 'loading'}
  <div class="loading"></div>
{:else}

  <!-- Flex -->
  <div class="flex-col gap-16">

    <!-- Message -->
    {#if status}<Message message={status} type="error" />{/if}

    <!-- Header -->
    <Header {pathname} {result} />

    <!-- Grid -->
    {#if result?.media}
      <div
        class="grid-container layout"
        bind:clientWidth={layoutWidth}
      >

          <!-- Left -->
          {#if showSides && (result?.comments?.length || result?.users?.length)}
            <div class="grid-left">

              <!-- Comments -->
              {#if result?.comments?.length}
                <Comments
                  comments={result?.comments}
                  media={result.media}
                />
              {/if}

            </div>
          {/if}

          <!-- Center -->
          <div
            class="grid-center"
            bind:clientWidth={centerWidth}
          >

            <!-- Media Large -->
            <div class="wrapper">
              <img
                alt={result.media?.caption || ''}
                class="image"
                class:radius-16={centerWidth + 2 < layoutWidth}
                draggable="false"
                src={getImageUrl(result.media?.media_id, 'large')}
              />
            </div>

            <!-- Caption -->
            <div class="caption">{result.media.caption}</div>

            <!-- Actions -->
            <div class={`actions-${showSides ? 'out' : 'in'}`}>
              <div>Four</div>
              <div>Three</div>
              <div>Two</div>
              <a href="/u/{result.media.user.user_id}" use:link>
                <UserPic size={50} user={result.media.user} />
              </a>
            </div>

          </div>

          <!-- Replies -->
          {#if showSides && result?.replies?.length}
            <div class="grid-right">
              <Replies media={result.replies} />
            </div>
          {/if}

      </div>
    {/if}

    <!-- Suggestions -->
    <div class="flex-col gap-16 {result?.media ? 'layout-media-margin' : 'layout-margin'}">

      <!-- Suggested Texts -->
      {#if result?.suggested_texts?.length}
        <SuggestedTexts {pathname} texts={result.suggested_texts} />
      {/if}

      <!-- Suggested Users -->
      {#if result?.users?.length && pathname !== 'u'}
        <SuggestedUsers users={result.users} />
      {/if}

    </div>

    <!-- Suggested Media -->
    {#if result?.suggested_media?.length}
      <SuggestedMedia media={result.suggested_media} />
    {/if}

  </div>

{/if}

<style>
  .grid-container {
    align-items: start;
    display: grid;
    grid-template-columns: 1fr min(100%, calc(95vh * 9 / 16)) 1fr;
  }

  .grid-center {
    aspect-ratio: 9 / 16;
    grid-column: 2;
    position: relative;
  }

  .grid-left,
  .grid-right {
    box-sizing: border-box;
    opacity: .85;
    overflow-x: hidden;
    overflow-y: auto;
    max-height: 95dvh;
  }

  .grid-left {
    border: 1px solid #f0f0f0;
    border-radius: 4px;
    margin: 0 16px 0 8px;
    padding: 8px;
  }

  .grid-right {
    align-items: flex-end;
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: 0 8px 0 64px;
  }

  .wrapper {
    inset: 0;
    overflow: hidden;
    position: absolute;
  }

  .image {
    display: block;
    height: 100%;
    object-fit: cover;
    object-position: center;
    width: 100%;
  }

  .caption {
    bottom: 0;
    color: white;
    font-size: 15px;
    font-weight: bold;
    left: 0;
    margin: 8px;
    position: absolute;
  }

  .actions-in,
  .actions-out {
    position: absolute;
    color: red;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .actions-in {
    bottom: 16px;
    right: 16px;
  }
  .actions-out {
    bottom: 0;
    left: calc(100% + 16px);
  }
</style>

<script>
  import { untrack } from 'svelte';
  import Comments from '../Comment/Comments.svelte';
  import Header from './Header.svelte';
  import Message from '../Misc/Message.svelte';
  import Replies from './Replies.svelte';
  import SuggestedMedia from './SuggestedMedia.svelte';
  import SuggestedTexts from './SuggestedTexts.svelte';
  import SuggestedUsers from './SuggestedUsers.svelte';
  import UserPic from '../User/UserPic.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';
  import link from '../Router/link.svelte';
  import redirect from '../Router/redirect.svelte';
  import store from '../lib/store.svelte';

  let
    centerWidth = $state(0),
    layoutWidth = $state(0),
    page = $state(1),
    result = $state(),
    showSides = $derived((layoutWidth - centerWidth) / 2 >= 200),
    status = $state();

  const pathname = $derived(store.url?.pathname?.split('/')?.[1]);

  const getUsers = result => {
    let users = [
      ...new Map(
        [
          ...(result.replies ?? []),
          ...(result.comments ?? []),
          ...(result.suggested_media ?? []),
        ]
          .map(it => it?.user)
          .filter(u => u?.user_id)
          .map(u => [u.user_id, u])
      ).values()
    ];
    const len = users.length;
    for (let i = 0; i < len; i++) {
      const j = Math.floor(Math.random() * (len - i)) + i;
      [users[i], users[j]] = [users[j], users[i]];
    }
    return users.slice(0, 20);
  };

  // Get Media
  const getMedia = async more => {
    if (status === 'loading') return;
    page = more ? page + 1 : 1;
    let payload = { page };
    if (pathname === 'explore') payload.type = 'explore';
    if (pathname === 'following') {
      if (!store.user?.user_id) {
        redirect('/explore', { replace: true });
        return;
      }
      payload.type = 'following';
    }
    if (pathname === 'm') payload = { type: 'media', value: store.url?.pathname.slice(3), ...payload };
    if (pathname === 's') payload = { type: 'text', value: decodeURIComponent(store.url?.pathname.slice(3)), ...payload };
    if (pathname === 'u') payload = { type: 'user', value: store.url?.pathname.slice(3), ...payload };
    if (payload.type === 'text' && payload?.value?.length < 3) return;
    if (!payload.type) return;
    status = 'loading';
    try {
      result = await fetchApi('get', '/media', payload);
      if (result?.suggested_media?.length) {
        const seen = new Set();
        result.suggested_media = result.suggested_media.filter(m => !seen.has(m.media_id) && seen.add(m.media_id));
      }
      result.users = getUsers(result);
      result.replies = result.suggested_media; // todo: delete
      result.comments = [{ comment_id: '1', user: result.suggested_media?.[0]?.user, text: 'First comment' }]; // todo: delete
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // Track store.url.pathname
  $effect(() => {
    store.url?.pathname;
    untrack(() => getMedia());
  });
</script>