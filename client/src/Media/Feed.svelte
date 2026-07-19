<!-- Feed -->
{#each store.feed as context, contextIndex (`${context.tab}:${contextIndex}`)}

  <!-- Context -->
  <Context {closeContext} {context} {contextIndex} />

  <ul class="grid reset{store.enlarged?.media && !store.isMobile ? ' aside' : ''}">
    {#each context.media as media, mediaIndex (`${media.media_id}:${mediaIndex}`)}

      <!-- Media -->
      <li class="flex-col gap-8 reset" id="{contextIndex}:{mediaIndex}">

        <a class="reset wrapper" href="." onclick={event => enlargeMedia(contextIndex, mediaIndex, event)}>

          <!-- Image -->
          <img
            alt={media.caption || ''}
            draggable="false"
            loading="lazy"
            src={getImageUrl(media.media_id, 'medium')}
          />

          <!-- Overlay -->
          <div class="overlay">

            <!-- User -->
            <div
              aria-hidden="true"
              class="user"
              onclick={() => newContext(`/u/${store.enlarged?.media?.user?.user_id}:user_media`)}
            >
              <UserPic size={16} user={media.user} />
              <div class="name white">{media.user?.name}</div>
            </div>

          </div>

          <!-- active -->
          {#if store.enlarged?.contextIndex === contextIndex && store.enlarged?.mediaIndex === mediaIndex}
            <div class="active"></div>
          {/if}

        </a>

        <!-- Caption -->
        {#if !store.isMobile && media.caption}
          <div class="caption reset text-primary">{media.caption}</div>
        {/if}

      </li>

    {/each}
  </ul>

{/each}

<style>
  .grid {
    --cols: 3;
    --gap: 1px;
    --padding: 0;
    --radius: 0;
    box-sizing: border-box;
    display: grid;
    _min-height: 0;
    min-width: 0;
    gap: var(--gap);
    grid-template-columns: repeat(var(--cols), minmax(0, 1fr));
    margin-inline: auto;
    padding: var(--padding);
    position: relative;
  }

  .grid.aside {
    --cols: 2;
    --gap: 16px;
    --padding: 8px;
    --radius: 8px;
  }

  @container (min-width: 400px) {
    .grid.aside {
      --cols: 3;
    }
  }

  .wrapper {
    aspect-ratio: 3 / 4;
    border-radius: var(--radius);
    _overflow: hidden;
    position: relative;

  }

  @media (min-width: 700px) {
    .grid {
      --cols: 4;
    }
  }

  @media (min-width: 1024px) {
    .grid {
      --cols: 5;
      --gap: 16px;
      --radius: 8px;
    }
  }

  @media (min-width: 1280px) {
    .grid {
      --cols: 6;
      --gap: 16px;
      --radius: 8px;
    }
  }

  img {
    border-radius: var(--radius);
    height: 100%;
    object-fit: cover;
    object-position: center;
    width: 100%;
  }

  .active {
    border: 3px solid var(--accent);
    border-radius: var(--radius);
    bottom: 0;
    box-sizing: border-box;
    height: 100%;
    left: 0;
    position: absolute;
    right: 0;
    top: 0;
  }

  .caption,
  .name {
    font-size: 14px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: 100%;
  }

  .overlay {
    background: linear-gradient(to top, #00000099 0%, transparent 100%);
    border-radius: var(--radius);
    bottom: 0;
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

  .user {
    align-items: center;
    display: flex;
    gap: 4px;
    overflow: hidden;
    width: 100%;
  }
</style>

<script>
  import Context from './Context.svelte';
  import UserPic from '../User/UserPic.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';
  import store from '../lib/store.svelte';

  const {
    closeContext,
    enlargeMedia,
    newContext,
  } = $props();
</script>
