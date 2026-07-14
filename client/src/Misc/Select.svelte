<div class="input-wrapper">
  <select
    class={[class_, 'input']}
    {id}
    onblur={() => { focused = false; }}
    onfocus={() => { focused = true; }}
    bind:this={select}
    bind:value={value}
    {...props}
  >
    {@render children()}
  </select>
  <label
    class="input-label"
    class:input-floating={focused || value}
    for={id}
  >
    {label}
  </label>
</div>

<script>
  import { untrack } from 'svelte';

  let {
    children,
    class: class_,
    id = `select-${Math.random().toString(36).slice(2)}`,
    label = undefined,
    value = $bindable(),
    ...props
  } = $props();

  let
    focused = $state(false),
    select = $state();

  $effect(() => untrack(() => {
    if (props.focus) select.focus();
  }));
</script>