<br><br>
<div class="layout-margin">
  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <!-- Message -->
    {#if status}<Message message={status} type="error" />{/if}

    <!-- Form -->
    <form onsubmit={login}>
      <div class="vertical-line">
        <span class="bold">Login to access your notes</span><br>
        Use a private window if not on your computer<br>
        Don't share your password
      </div>
      <br><br><br>
      <div class="flex-col align-flex-start gap-8">

        <!-- Email -->
        <Input
          focus
          label="E-mail"
          minlength="10"
          maxlength="100"
          required
          size="40"
          tabindex={1}
          type="email"
          bind:value={user.email}
        />

        <!-- Password -->
        <Input
          label="Password"
          minlength="8"
          maxlength="30"
          size="40"
          required
          tabindex={2}
          type="password"
          bind:value={user.password}
        />
      </div>
      <br><br>
      <div class="flex gap-8{store.columnCount === 1 ? ' justify-end' : ''}">
        <button
          class="button-white"
          onclick={() => redirect('/register')}
          tabindex={4}
          type="button"
        >Create account</button>
        <button
          class="button-blue"
          tabindex={3}
          type="submit"
        >Login</button>
      </div>
    </form>

  {/if}
</div>

<script>
  import { untrack } from 'svelte';
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import redirect from '../Router/redirect.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  let
    status = $state(),
    user = $state({});

  const login = async e => {
    e.preventDefault();
    status = 'loading'
    localStorage.clear();
    store.individual = undefined;
    store.timeclock = undefined;
    try {
      // Login
      const res = await fetchApi('post', '/auth/login', {
        email: user.email,
        password: user.password,
      });

      // Session
      if (res.tokens) {
        localStorage.setItem('access_token', res.tokens.access_token);
        localStorage.setItem('refresh_token', res.tokens.refresh_token);
        localStorage.setItem('user', JSON.stringify(res.user));
        store.user = res.user;
      }

      store.cache = new Map();
      store.tabs = new Map();
      redirect('/explore', { replace: true });
    } catch (e) {
      status = e.message;
    }
  };

  // Track store.user
  $effect(() => {
    store.user?.user_id;
    untrack(() => store.user?.user_id && redirect('/explore', { replace: true }));
    // untrack(() => { if (store.user?.user_id) redirect('/explore', { replace: true }); });
  });
</script>