<br><br>
<div class="layout-margin">
  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <!-- Message -->
    {#if status}
      <Message message={message} type={status} />
      <br><br>
    {/if}

    <!-- Form -->
    {#if user}
      <form onsubmit={patchUser}>

        <!-- Name -->
        <div class="inline-flex-col gap-8">
          <Input
            label="First name"
            maxlength="50"
            required
            size="40"
            tabindex={1}
            bind:value={user.first_name}
          />
          <Input
            label="Last name"
            maxlength="50"
            required
            size="40"
            tabindex={2}
            bind:value={user.last_name}
          />
        </div>

        <!-- E-mail -->
        <br><br><br><br>
        <div class="inline-flex gap-8">
          <Input
            label="E-mail"
            disabled
            minlength="10"
            maxlength="100"
            readonly
            required
            size="40"
            type="email"
            bind:value={user.email}
          />
          <button
            class="button-white"
            onclick={e => dialog = 'email'}
            type="button"
          >Change</button>
          {#if dialog === 'email'}
            <Email bind:dialog bind:user />
          {/if}
        </div>

        <!-- Password -->
        <br><br><br><br>
        <div class="inline-flex gap-8">
          <button
            class="button-white"
            onclick={changePassword}
            type="button"
          >Change Password</button>
        </div>

        <!-- MFA -->
        <br><br><br><br>
        <div class="inline-flex-col gap-8">
          <label class="label">
            <input
              checked={user.mfa}
              onchange={() => user.mfa = !user.mfa}
              tabindex={3}
              type="checkbox"
            >
            <div class="bold blue-1">Multi Factor Authentication</div>
          </label>
        </div>

        <!-- Buttons -->
        <br><br><br>
        <div class="flex gap-8{store.columnCount === 1 ? ' justify-end' : ''}">
          <button class="button-blue" tabindex={4} type="submit">Save</button>
        </div>

      </form>
    {/if}

  {/if}
</div>

<script>
  import { untrack } from 'svelte';
  import Email from './Email.svelte';
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';
  import redirect from '../Router/redirect.svelte';

  let
    dialog = $state(),
    message = $state(),
    status = $state(),
    user = $state(/*{ mfa: true, first_name: '', last_name: '', email: '' }*/);

  const changePassword = e => {
    e.preventDefault();
    redirect('/password');
  }

  const getUser = async () => {
    if (status !== 'loading') {
      status = 'loading';
      try {
        const res = await fetchApi('get', '/users');
        if (res.user) {
          store.user = {
            password_salt: store.user.password_salt,
            key: store.user.key,
            ...res.user,
          };
          user = res.user;
        }
        message = status = undefined;
      } catch (e) {
        message = e.message;
        status = 'error';
      }
    }
  };

  const patchUser = async e => {
    e.preventDefault();
    if (status !== 'loading') {
      status = 'loading';
      try {
        await fetchApi('patch', '/users', {
          mfa: user.mfa,
          first_name: user.first_name,
          last_name: user.last_name,
        });
        message = 'Updated';
        status = 'info';
      } catch (e) {
        message = e.message;
        status = 'error';
      }
    }
  };

  // mount
  $effect(() => untrack(() => getUser()));
</script>