<br><br>
<div class="layout-margin">
  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <!-- Message -->
    {#if status}
      <Message message={status} type="error" />
      <br><br>
    {/if}

    <!-- Form -->
    <form onsubmit={updatePassword}>
      <div class="vertical-line">
        <span class="bold">Current password</span><br>
        Enter your current password for verification<br>
        Use a private window if not on your computer
      </div>
      <br><br>
      <div class="flex-col align-flex-start gap-8">
        <Input
          focus
          label="Current password"
          minlength="8"
          maxlength="30"
          required
          size="20"
          tabindex={1}
          type="password"
          bind:value={user.password}
        />
      </div>
      <br><br><br>
      <div class="vertical-line">
        <span class="bold">New password</span><br>
        Create a new password for your account<br>
        At least 8 characters long
      </div>
      <br><br>
      <div class="flex-col align-flex-start gap-8">
        <Input
          label="New password"
          minlength="8"
          maxlength="30"
          required
          size="20"
          tabindex={2}
          type="password"
          bind:value={user.new_password}
        />
        <Input
          label="Re-enter new password"
          minlength="8"
          maxlength="30"
          required
          size="20"
          tabindex={3}
          type="password"
          bind:value={user.rePassword}
        />
      </div>
      <br><br><br>
      <div class="flex gap-8{store.columnCount === 1 ? ' justify-end' : ''}">
        <button
          class="button-white"
          onclick={() => redirect('/account')}
          tabindex={5}
          type="button"
        >Cancel</button>
        <button
          class="button-blue"
          tabindex={4}
          type="submit"
        >Update Password</button>
      </div>
    </form>

  {/if}
</div>

<script>
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import redirect from '../Router/redirect.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  let
    status = $state(),
    user = $state({});

  const updatePassword = async e => {
    e.preventDefault();
    status = 'loading'
    try {

      // Check password
      if (user.new_password !== user.rePassword) {
        throw new Error('Passwords don\'t match');
      }

      // update password
      const res = await fetchApi('patch', '/auth/login', {
        password: user.password,
        new_password: user.new_password,
      });

      localStorage.setItem('user', JSON.stringify(store.user));
      redirect('/account', { replace: true });
    } catch (e) {
      status = e.message;
    }
  }
</script>