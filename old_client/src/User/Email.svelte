<Dialog bind:dialog>
  {#if status === 'loading'}
    <div class="loading"></div>
    {:else}

    <div class="flex-col gap-8 padding-8">

      <div class="bold font-16 gray-1">Update E-mail</div>
      <br>

      <!-- Message -->
      {#if status}<Message message={status} type="error" />{/if}

      <!-- Email, Captcha (MFA) -->
      {#if draft.step === 1}
        <form onsubmit={mfaEmail}>
          <Input
            focus
            label="New e-mail"
            max
            minlength="10"
            maxlength="100"
            required
            size="40"
            tabindex={1}
            type="email"
            oninput={() => { draft.email = draft.email.trim().toLowerCase(); }}
            bind:value={draft.email}
          />
          <br><br>
          <div class="flex gap-8{store.columnCount === 1 ? ' justify-end' : ''}">
            <button
              class="button-white"
              onclick={() => { dialog = undefined; }}
              tabindex={4}
              type="button"
            >Cancel</button>
            <button
              class="button-blue"
              tabindex={3}
              type="submit"
            >Next</button>
          </div>
        </form>

      <!-- MFA Code -->
      {:else if draft.step === 2}
        <form onsubmit={mfaCode}>
          <Input
            focus
            label="New e-mail"
            max
            minlength="10"
            maxlength="100"
            readonly
            required
            size="40"
            tabindex={1}
            type="email"
            oninput={() => { draft.email = draft.email?.trim().toLowerCase(); }}
            bind:value={draft.email}
          />
          <br><br>
          <Input
            label="Code"
            minlength="6"
            maxlength="6"
            required
            size="6"
            tabindex={2}
            oninput={() => { draft.code = draft.code?.trim().toLowerCase(); }}
            bind:value={draft.code}
          />
          <br><br>
          <div class="flex gap-8{store.columnCount === 1 ? ' justify-end' : ''}">
            <button
              class="button-white"
              onclick={() => { draft.step = 1; }}
              tabindex={4}
              type="button"
            >Back</button>
            <button
              class="button-blue"
              tabindex={3}
              type="submit"
            >Confirm Code</button>
          </div>
        </form>

      {/if}

    </div>
  {/if}
</Dialog>

<script>
  import Dialog from '../Dialog/Dialog.svelte';
  import Input from '../Misc/Input.svelte';
  import Message from '../Misc/Message.svelte';
  import fetchApi from '../lib/fetchApi.svelte';
  import store from '../lib/store.svelte';

  let
    {
      dialog = $bindable(),
      user = $bindable(),
    } = $props(),
    draft = $state({ step: 1 }),
    status = $state();

  const mfaEmail = async e => {
    e.preventDefault();
    if (status !== 'loading') {
      status = 'loading';
      try {
        const res = await fetchApi('patch', '/users/email', { email: draft.email });
        draft = { step: 2, email: draft.email };
        status = undefined;
      } catch (e) {
        draft = { step: 1, email: draft.email };
        status = e.message;
      }
    }
  };

  const mfaCode = async e => {
    e.preventDefault();
    if (status !== 'loading') {
      status = 'loading';
      try {
        draft.code = parseInt(draft.code) ?? '';
        const res = await fetchApi('patch', '/users/email', { email: draft.email, code: draft.code });
        user.email = draft.email;
        dialog = status = undefined;
      } catch (e) {
        draft = { step: 2, email: draft.email };
        status = e.message;
      }
    }
  };
</script>