<div
  class="input-wrapper"
  class:width-100={max}
>
  <input
    class={[class_, 'input']}
    {id}
    onblur={() => { focused = false; }}
    onfocus={() => { focused = true; }}
    bind:this={input}
    bind:value={value}
    {...props}
  />
  <label
    class="input-label"
    class:input-floating={props.type === 'date' || focused || value}
    for={id}
  >
    {label}
  </label>
</div>

<script>
  import { untrack } from 'svelte';

  let {
    class: class_,
    id = `input-${Math.random().toString(36).slice(2)}`,
    label = undefined,
    max = false,
    value = $bindable(),
    ...props
  } = $props();

  let
    focused = $state(false),
    input = $state();

  $effect(() => untrack(() => {
    if (props.focus) input?.focus();
  }));
</script>