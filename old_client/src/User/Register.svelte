<br><br>
<div class="layout-margin">
  {#if status === 'loading'}
    <div class="loading"></div>
  {:else}

    <!-- Message -->
    {#if status}<Message message={status} type="error" />{/if}

    <form class="align-flex-start flex-col gap-8" {onsubmit}>

      {#if draft.verify}

        <!-- Code -->
        <Input
          focus
          label="Code"
          minlength="8"
          maxlength="8"
          required
          size="8"
          tabindex={1}
          oninput={() => { draft.code = draft.code?.trim().toUpperCase(); }}
          bind:value={draft.code}
        />

        <!-- Button -->
        <Button
          label="Confirm E-mail"
          tabindex={2}
          type="submit"
        />

      {:else}

        <!-- Full name -->
        <Input
          focus
          label="Full name"
          minlength="5"
          maxlength="50"
          required
          size="40"
          tabindex={1}
          bind:value={draft.name}
        />

        <!-- E-mail -->
        <Input
          label="E-mail"
          minlength="10"
          maxlength="100"
          required
          size="40"
          tabindex={2}
          type="email"
          oninput={() => { draft.email = draft.email?.trim().toLowerCase(); }}
          bind:value={draft.email}
        />

        <!-- Password -->
        <Input
          label="Create password"
          minlength="8"
          maxlength="30"
          required
          size="20"
          tabindex={3}
          type="password"
          bind:value={draft.password}
        />

        <!-- Birth date -->
        <Input
          label="Birth date"
          required
          tabindex={4}
          type="date"
          bind:value={draft.birth_date}
        />

        <!-- Phone (optional) -->
        <Input
          label="Phone (optional)"
          minlength="10"
          maxlength="15"
          size="20"
          tabindex={5}
          type="tel"
          bind:value={draft.phone}
        />

        <!-- Terms -->
        <label class="label">
          <input
            checked
            onclick={e => e.preventDefault()}
            type="checkbox"
          >
          <span class="bold">I AGREE</span> width the <a href="/terms" target="_blank" use:link>Terms and Conditions</a>
        </label>

        <!-- Button -->
        <Button
          label="Create account"
          tabindex={6}
          type="submit"
        />

      {/if}

    </form>

  {/if}
</div>

<script>
  import Button from '../Misc/Button.svelte';
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import redirect from '../Router/redirect.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import link from '../Router/link.svelte';
  import store from '../lib/store.svelte';

  let
    draft = $state({}),
    status = $state();

  const onsubmit = async e => {
    e.preventDefault();
    if (status === 'loading') return;
    status = 'loading';
    try {
      const body = {
        name: draft.name,
        email: draft.email,
        password: draft.password,
        phone: draft.phone || undefined,
        birth_date: draft.birth_date,
        country: 'US',
        code: draft.code,
      }
      const res = await fetchApi('post', '/users', body);

      if (!res.tokens) {
        draft.verify = true;
        status = undefined;
        return;
      }

      localStorage.setItem('access_token', res.tokens.access_token);
      localStorage.setItem('refresh_token', res.tokens.refresh_token);
      localStorage.setItem('user', JSON.stringify(res.user));
      store.user = res.user;

      store.cache = new Map();
      store.tabs = new Map();
      redirect('/explore', { replace: true });
    } catch (e) {
      status = e.message;
    }
  };
</script>