{#if comments?.length}
  <ul class="flex-col flex-grow gap-16 reset">
    {#each comments as it (it.comment_id)}
      <li
        class="flex gap-8 reset"
        style="line-height: 25px;"
      >
        <a href="/u/{it.user?.user_id}" use:link>
          <UserPic
            size={25}
            user={it.user}
          />
        </a>
        <div>
          <a href="/u/{it.user?.user_id}" use:link>
            <b>{it.user?.name}</b>
          </a>:
          {it.text}
          <span>{formatTimestamp(it.created_at)}</span>
          {#if store.user?.user_id && (it.user.user_id === store.user?.user_id || result.media?.user.user_id === store.user?.user_id)}
            <span class="red">x</span>
          {/if}
          {#if store.user?.user_id}
            <span class="orange">!</span>
          {/if}
        </div>
      </li>
    {/each}
  </ul>
{/if}

<script>
  import { untrack } from 'svelte';
  import UserPic from '../User/UserPic.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import { formatTimestamp } from '../lib/format';
  import link from '../Router/link.svelte';
  import store from '../lib/store.svelte';

  let
    { result = $bindable() } = $props(),
    comments = $state.raw(),
    status = $state();

  // Get Comments
  const getComments = async more => {
    if (status === 'loading') return;
    const uuid = store.url?.pathname?.slice(3);
    const key = `comments-${uuid}`;
    let comments = store.getCache(key) || [];
    const offset = comments?.length ?? 0;
    // Use cached
    if (offset && !more) return;
    // Fetch
    const payload = { offset };
    status = 'loading';
    try {
      const res = await fetchApi('get', `/comments/${uuid}`, payload);
      if (!res.comments) res.comments = [];
      comments = more ? [...comments, ...res.comments] : res.comments;
      store.setCache(key, comments, more);
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // Mount
  $effect(() => untrack(() => getComments()));
</script>