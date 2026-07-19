import store from './store.svelte';

export default (media_id, variant) => `${store.CDN}${media_id}/${variant}`;