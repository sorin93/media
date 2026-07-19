<div class="context{!store.isMobile ? ' sticky' : ''}">
  {#if context.tab?.startsWith('/u/')}
    <UserPic size={32} user={context.subject} />
    <span class="flex-grow">{context.subject?.name}</span>
  {:else if context.tab?.startsWith('/m/')}
    <Svg
      class="pointer"
      fill="var(--fill)"
      name="image_search"
      size="30px"
    />
    <span class="flex-grow">Suggested</span>
  {:else if context.tab?.startsWith('/c/')}
    <Svg
      class="pointer"
      fill="var(--fill)"
      name="comment"
      size="30px"
    />
    <span class="flex-grow">Comments</span>
  {:else if context.tab?.startsWith('/s/')}
    <Svg
      class="pointer"
      fill="var(--fill)"
      name="search"
      size="30px"
    />
    <span class="flex-grow">{context.tab.slice(3)}</span>
  {:else if context.tab === '/explore:'}
    <Svg
      class="pointer"
      fill="var(--fill)"
      name="explore"
      size="30px"
    />
    <span class="flex-grow">Explore</span>
  {:else if context.tab === '/following:'}
    <Svg
      class="pointer"
      fill="var(--fill)"
      name="group"
      size="30px"
    />
    <span class="flex-grow">{context.tab.slice(3)}</span>
  {/if}

  <!-- Close -->
  {#if contextIndex}
    <a href="." onclick={event => closeContext(contextIndex, event)}>
      <Svg
        fill="var(--fill)"
        name="close"
        size="30px"
      />
    </a>
  {/if}
</div>

<style>
  .context {
    align-items: center;
    background: inherit;
    box-sizing: border-box;
    display: flex;
    font-weight: bold;
    gap: 16px;
    padding: 8px;
  }

  .sticky {
    position: sticky;
    top: 0;
    z-index: 2;
  }
</style>

<script>
  import Svg from '../Svg/Svg.svelte';
  import UserPic from '../User/UserPic.svelte';
  import store from '../lib/store.svelte';

  const {
    closeContext,
    context,
    contextIndex,
  } = $props();
</script>
