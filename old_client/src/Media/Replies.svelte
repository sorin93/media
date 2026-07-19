<!-- Grid -->
<div class="grid">
  {#each media as it (it.media_id)}

    <!-- Item -->
    <a
      href="/m/{it.media_id}"
      title={it.caption || it.user?.name || ''}
      use:link
    >

      <!-- Media -->
      <img
        alt={it.caption || it.user?.name || ''}
        class="tile"
        draggable="false"
        src={getImageUrl(it.media_id, 'medium')}
      />

      <!-- Overlay -->
      <div class="overlay">
        {(it.comment_count + it.reply_count) || ''}
      </div>

    </a>

  {/each}
</div>

<style>
  .grid {
    display: grid;
    gap: 16px;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  a {
    aspect-ratio: 1;
    border-radius: 4px;
    overflow: hidden;
    position: relative;
  }

  img {
    height: 100%;
    object-fit: cover;
    object-position: center;
    width: 100%;
  }

  .overlay {
    background: linear-gradient(to top, #00000099 0%, transparent 100%);
    bottom: 0;
    color: white;
    display: flex;
    flex-direction: column;
    font-size: 14px;
    gap: 8px;
    height: 40%;
    justify-content: flex-end;
    left: 0;
    overflow: hidden;
    padding: 4px;
    position: absolute;
    right: 0;
  }
</style>


<script>
  import UserPic from '../User/UserPic.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';
  import link from '../Router/link.svelte';

  const { media } = $props();
</script>