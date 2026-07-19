<textarea
  class="input-1"
  {maxlength}
  minlength="1"
  {placeholder}
  required
  rows="1"
  {oninput}
  {onkeydown}
  tabindex={1}
  bind:value={value}
  bind:this={textarea}
  {...props}
></textarea>

<script>
  import { untrack } from 'svelte';
  import store from '../lib/store.svelte';

  let {
    focus,
    maxlength = 50,
    onsubmit,
    placeholder= 'Task...',
    value = $bindable(),
    ...props
  } = $props();

  let textarea = $state();

  const resize = () => {
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = textarea.scrollHeight + 'px';
    }
  };

  const oninput = () => {
    // value = value.replace(/\r?\n/g, '');
    resize();
  };

  const onkeydown = e => {
    if (e?.key === 'Enter') onsubmit();
  };

  // Track draft.name, store.width
  $effect(() => {
    value;
    store.width;  // todo: i dont have store.width anymore
    untrack(() => resize());
  });

  $effect(() => {
    if (focus) textarea?.focus();
  });
</script>