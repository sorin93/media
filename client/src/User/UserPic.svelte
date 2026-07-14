<div
  style="border-radius: 50%; height: {size}px; width: {size}px;"
>
  {#if user?.media_id && !error}
    <img
      alt={user.name || ''}
      loading="lazy"
      {onerror}
      src={getImageUrl(user.media_id, 'small')}
    />
  {:else}
    <Svg name="person" {size} />
  {/if}
</div>

<style>
  div {
    aspect-ratio: 1;
    background-color: var(--light-gray, #e0e0e0);
    display: block;
    overflow: hidden;
    user-select: none;
  }

  img {
    display: block;
    height: 100%;
    object-fit: cover;
    object-position: center;
    width: 100%;
  }
</style>

<script>
  import Svg from '../Svg/Svg.svelte';
  import getImageUrl from '../lib/getImageUrl.svelte';

  const { size, user } = $props();

  let error = $state(false);

  const onerror = () => { error = true; };

  $effect(() => {
    user;
    error = false;
  });
</script>