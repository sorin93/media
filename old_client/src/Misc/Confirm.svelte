{#if dialog}
  <Dialog bind:dialog title={label}>
    <div class="flex-col gap-16 padding-16">
      <div class="bold red-2">{label}</div>
      <div class="justify-end flex gap-16">
        <button class="button-white" onclick={() => dialog = undefined}>Cancel</button>
        <button class="button-blue" {onclick}>OK</button>
      </div>
    </div>
  </Dialog>
{/if}

<script>
  import Dialog from '../Dialog/Dialog.svelte'

  let {
    dialog = $bindable(),
    label,
    onclick,
  } = $props();
</script>