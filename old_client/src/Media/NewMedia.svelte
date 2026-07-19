<br><br><br><br>
<div class="layout-margin">
  <div>New Media</div>

  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <!-- Message -->
    {#if status}<Message message={status} type="error" />{/if}

    <!-- Form -->
    <form class="flex-col gap-16" {onsubmit}>

      <!-- Image -->
      <input
        accept="image/*, video/*"
        required
        tabindex={1}
        type="file"
        bind:files
      >

      <!-- Caption -->
      <Input
        label="Caption"
        maxlength="100"
        size="50"
        tabindex={2}
        bind:value={draft.caption}
      />

      <!-- Button -->
      <div>
        <Button
          label="Upload"
          tabindex={2}
          type="submit"
        />
      </div>

    </form>

  {/if}

  </div>

<script>
  import Button from '../Misc/Button.svelte';
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import redirect from '../Router/redirect.svelte';

  let
    draft = $state({}),
    files = $state.raw(),
    status = $state();

  const onsubmit = async e => {
    e.preventDefault();
    if (status === 'loading' || files?.[0]?.type?.slice(0, 5) !== 'image') return;
    status = 'loading';
    try {
      const body = new FormData();
      if (draft.parent_media_id) body.append('parent_media_id', draft.parent_media_id);
      body.append('type', files?.[0].type);
      draft.caption = draft.caption?.trim();
      if (draft.caption) body.append('caption', draft.caption);
      body.append('file', files[0]);
      await fetchApi('get', '/auth/me');
      const access_token = localStorage.getItem('access_token');
      let res = await fetch('https://post.sorin.cc', {
        credentials: 'include',
        headers: {
          'Authorization': `Bearer ${access_token}`,
        },
        method: 'POST',
        body,
        mode: 'cors',
      });
      if (!res.ok) throw new Error(await res.text());
      res = await res.json();
      redirect(`/m/${res?.media?.media_id}`, { replace: true });
      status = undefined;
    } catch (e) {
      status = e.message || 'Error';
    }
  };
</script>