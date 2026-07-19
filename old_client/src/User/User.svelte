<br><br><br><br>
<div class="layout-margin">

  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <h1>User Media</h1>

    <!-- Message -->
    {#if status}<Message message={status} type="error" />{/if}

    <!-- New Media -->
    <a href="/media/new" use:link>New Media</a>

    <br><br>

    <!-- Media -->
    {#if media.length}
      <div class="flex-col gap-8">
        {#each media as m}
          <div class="flex gap-8">
            <!-- todo there is not public or small anymore -->
            <a href="/media/{m.media_id}/single" use:link>
              {#if m.type === 'image'}
                <!--
                <img
                  alt=""
                  src={m.url.replace('/public', '/small')}
                >
                -->
              {/if}
            </a>
            <a href="/media/{m.media_id}/multiple" use:link>multiple</a>
          </div>
        {/each}
      </div>
    {:else}
      No media
    {/if}

  {/if}
</div>

<script>
  import { untrack } from 'svelte';
  import Message from '../Misc/Message.svelte';
  import Svg from '../Svg/Svg.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import { formatTimestamp } from '../lib/format';
  import link from '../Router/link.svelte';
  import store from '../lib/store.svelte';
  import { UUID } from '../lib/regexp';

  const RE = new RegExp(`^/users/(?<user_id>${UUID})/?$`);

  let
    media = $state([]),
    status = $state();

  const getUserMedia = async () => {
    const user_id = RE.exec(store.url?.pathname?.toLocaleLowerCase())?.groups?.user_id ?? store.user?.user_id;
    if (status === 'loading' || !user_id) return;
    status = 'loading';
    try {
      const res = await fetchApi('get', `/users/${user_id}/media`);
      media = res.media || [];
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };

  // Track store.url.pathname
  $effect(() => {
    store.url?.pathname;
    untrack(() => getUserMedia());
  });
</script>