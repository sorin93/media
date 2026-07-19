<nav class="flex-col gap-16 overflow-x padding-8">

  <div class="flex gap-16">
    {#each tabs as it}
      <a
        class="reset{it.split(':')?.[1] === tab.split(':')?.[1] ? ' bold' : ''}"
        href={it?.split(":")?.[0]}
        data-tab={it === tab ? undefined : it}
        use:link>
          {label(it)}
      </a>
    {/each}
  </div>
</nav>

<style>
  nav {
    background: inherit;
  }
</style>

<script>
  import link from "../Router/link.svelte";
  import store from '../lib/store.svelte';

  const { tab, tabs } = $props();

  const label = (tab) => {
    if (tab === "/explore:") return "Explore";
    if (tab === "/following:") return "Following";
    if (tab?.slice(0, 3) === "/s/") return "Search";
    const part = tab?.split(":")?.at(-1);
    if (part === "user_media") return "From User";
    if (part === "user_comments") return "Commented";
    if (part === "user_likes") return "Liked";
    if (part === "media_suggestions") return "Suggested";
    if (part === "media_comments") return "Comments";
    if (part === "media_likes") return "Likes";
    return "";
  };
</script>
