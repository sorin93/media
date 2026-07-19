{#if user}
  <address class="align-center flex gap-8">
    <UserPic size={48} {user} />
    <h1 class="reset">{user.name}</h1>
  </address>
{/if}

<script>
  import UserPic from '../User/UserPic.svelte';

  const { user } = $props();
</script>