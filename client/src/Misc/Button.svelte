<button class={[class_, format]} {...props}>{label}</button>

<style>
  button {
    align-items: center;
    border: 0;
    border-radius: 32px;
    box-sizing: border-box;
    cursor: pointer;
    display: inline-flex;
    font-family: Roboto;
    font-size: .875rem;
    font-weight: 500;
    gap: 4px;
    letter-spacing: 0.09;
    line-height: 40px;
    min-width: 100px;
    outline: none;
    overflow: hidden;
    padding: 0 15px;
    text-overflow: ellipsis;
    transition: opacity 200ms cubic-bezier(0.35, 0, 0.25, 1),background-color 200ms cubic-bezier(0.35, 0, 0.25, 1);
    user-select: none;
    white-space: nowrap;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -webkit-tap-highlight-color: transparent;  /* removes square selection */
    -webkit-touch-callout: none;
  }

  button:active {
    filter: opacity(0.24);
    transition: box-shadow 280ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  button:disabled {
    filter: grayscale(100%) opacity(0.5);
  }

  button:hover {
    filter: brightness(1.1);
  }

  .elevated {
    background-color: transparent;
    box-shadow: 0 2px 4px -1px #00000066;
  }

  .filled {
    background: var(--primary-color);
    color: white;
  }

  .outlined {
    background-color: transparent;
    border: 1px solid var(--primary-color);
    color: var(--primary-color);
  }

  .text {
    background-color: transparent;
    color: var(--primary-color);
  }

  .text:hover {
    background-color: #00000011;
  }
</style>

<script>
  let {
    class: class_,
    format,
    label,
    ...props
  } = $props();
</script>