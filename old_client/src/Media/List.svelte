<!-- Suggested Texts -->
{#if result?.texts?.length}
  <ul class="flex flex-wrap gap-8 reset" style="margin: 0 8px">
    {#each result.texts as it (it)}
      <li class="reset text">
        <a
          class="reset"
          href="/s/{encodeURIComponent(it)}"
          data-tab="/s/{it}:"
          use:link
        >{it}</a>
      </li>
    {/each}
  </ul>
{/if}

<!-- Suggested Media -->
<ul class="grid reset{enlarged && result?.media && !store.isMobile ? ' aside' : ''}">
  {#each result?.media || [] as it, i (it.media_id)}

    <!-- Item -->
    <li class="flex-col gap-8 reset" >

      <a class="reset wrapper" href="/m/{it.media_id}" data-replace use:link>

        <!-- Image -->
        <img
          alt={it.caption || ''}
          draggable="false"
          loading="lazy"
          src={getImageUrl(it.media_id, 'medium')}
        />

        <!-- Overlay -->
        <div class="overlay">

          <!-- User -->
          <div class="user">
            <UserPic size={16} user={it.user} />
            <div class="name white">{it.user?.name}</div>
          </div>

        </div>

        <!-- active -->
        {#if result?.index === i}
          <div class="active"></div>
        {/if}


      </a>

      <!-- Caption -->
      {#if !store.isMobile}
        <a class="caption reset text-primary" href="/m/{it.media_id}" data-replace use:link>{it.caption || ''}</a>
      {/if}

    </li>

  {/each}
</ul>

<style>
  .text {
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 4px;
    white-space: nowrap;
    /* todo */
  }

  .grid {
    --cols: 3;
    --gap: 1px;
    --padding: 0;
    --radius: 0;
    box-sizing: border-box;
    display: grid;
    min-height: 0;
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
  import link from '../Router/link.svelte';
  import UserPic from '../User/UserPic.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';
  import store from '../lib/store.svelte';

  const {
    enlarged = false,
    result = undefined,
    tab = undefined,
  } = $props();
</script>
