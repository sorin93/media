{#if message && type}
  <div class={type}>
    <Svg
      name={type === 'warning' ? 'error' : type}
      size="16px"
    />
    {message}
  </div>
{/if}

<style>
  div {
    align-items: center;
    border-radius: 4px;
    display: flex;
    font-size: 14px;
    gap: 8px;
    margin: 1px;
    padding: 12px 18px;
  }

  .error {
    background-color: #fed9d6;
    color: #420106;
    fill: #420106;
    margin-bottom: 16px;
  }

  .info {
    background-color: #e8f4fd;
    color: #0d3c61;
    fill: #0d3c61;
  }

  .success {
    background-color: #edf7ed;
    color: #1e4620;
    fill: #1e4620;
  }

  .warning {
    background-color: #fff4e5;
    color: #663c00;
    fill: #663c00;
  }
</style>

<script>
  import Svg from '../Svg/Svg.svelte';
  const { message, type } = $props();
</script>