<svelte:window onkeydown|capture={onKey} onpopstate={onclick} />

<!-- Scrim -->
<div
  aria-hidden="true"
  class="scrim"
  {onclick}
  transition:fade={{ duration: 25 }}
>

  <!-- Dialog -->
  <dialog
    class={[class_, 'dialog']}
    onclick={e => e.stopPropagation()}
    {...props}
  >

    <!-- Children -->
    {@render children()}

  </dialog>

</div>

<style>
  .scrim {
    align-items: center;
    backdrop-filter: blur(2px);
    background-color: #00000008;
    bottom: 0;
    display: flex;
    justify-content: center;
    left: 0;
    padding: 8px;
    position: fixed;
    right: 0;
    top: 0;
    z-index: 1002;
  }

  .dialog {
    background-color: white;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    box-shadow: 0 2px 4px #00000020;
    box-sizing: border-box;
    margin: auto;
    max-height: calc(100% - 16px);
    max-width: calc(100% - 16px);
    overflow-x: hidden;
    overflow-y: auto;
    padding: 16px 8px;
    position: absolute;
    width: 400px;
  }
</style>

<script>
	import { fade } from 'svelte/transition';

  let {
    class: class_,
    dialog = $bindable(),
    children,
    ...props
  } = $props();


  const onclick = e => {
    e?.preventDefault();
    e?.stopPropagation();
    dialog = undefined;
  };

  const onKey = e => {
    if (e.key === 'Escape' && (e.target.tagName.toUpperCase() === 'BODY')) {
      e.stopImmediatePropagation();
      onclick(e);
    }
  };
</script>