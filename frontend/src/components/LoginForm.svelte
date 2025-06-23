<script lang="ts">
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { notifCenter } from "@/stores/notifications";
  import Notifications from "./Notifications.svelte";

  let email: string;
  let password: string;

  function submit() {
    ApiRoutes.login({ email, password })
      .then((_) => {
        window.location.reload();
      })
      .catch((e) => {
        notifCenter.error(e);
      });
  }
</script>

<Notifications />
<form
  on:submit={(ev) => {
    ev.preventDefault();
    submit();
  }}
>
  <label
    >Email
    <input type="email" bind:value={email} required />
  </label>

  <label
    >Password
    <input type="password" bind:value={password} required />
  </label>

  <button type="submit">Login</button>
</form>
