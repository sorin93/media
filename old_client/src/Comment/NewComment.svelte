{#if status === 'loading'}
  <div class="loading"></div>
{:else}

  <!-- Message -->
  {#if status}
    <Message message={status} type='error' />
  {/if}

  <!-- Form -->
  <form {onsubmit}>

    <!-- Text -->
    <Textarea
      class="flex-grow"
      maxlength="100"
      placeholder="Comment"
      required
      tabindex={1}
      type="text"
      bind:value={text}
      {onsubmit}
    />

    <!-- Button -->
    <div>
      <Button
        label="Post"
        tabindex={2}
        type="submit"
      />
    </div>

  </form>

{/if}

<style>
  form {
    bottom: 0;
    display: flex;
    gap: 8px;
    left: 0;
    margin: 8px;
    _position: absolute;
    right: 0;
  }
</style>

<script>
  import Button from '../Misc/Button.svelte';
  import Textarea from '../Misc/Textarea.svelte';
  import Message from '../Misc/Message.svelte';
  import fetchApi from '../lib/fetchApi.svelte';

  let
    {
      comments = $bindable(undefined),
      medium = $bindable(),
    } = $props(),
    text = $state(''),
    status = $state();

  const onsubmit = async e => {
    e?.preventDefault();
    if (status === 'loading') return;
    status = 'loading';
    try {
      const
        payload = {
          media_id: medium.media_id,
          text,
        },
        res = await fetchApi('post', '/comments', payload);
      comments = [ ...(comments ?? []), res.comment];
      medium.comment_count += 1;
      medium = medium;
      text = '';
      status = undefined;
    } catch (e) {
      status = e.message;
    }
  };
</script>