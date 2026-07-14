<svg
  aria-hidden="true"
  {fill}
  height={size}
  viewBox="0 -960 960 960"
  width={size}
  xmlns="http://www.w3.org/2000/svg"
  {...props}
>
  {#if title}<title>{title}</title>{/if}
  {@html svg[name]}
</svg>

<style>
  svg {
    display: inline-block;
    vertical-align: middle;
  }

  /*
  svg {
    aspect-ratio: 1/1;
    box-sizing: border-box;
    max-width: calc(100% - 16px);
    width: 100%;
  }
  */

</style>

<script>
  import svg from './svg';

  let {
    fill = 'var(--text-secondary)',
    name,
    size = '24px',
    title,
    ...props
  } = $props();
</script>