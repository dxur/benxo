<script lang="ts">
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { onMount } from "svelte";
  import Notifications from "./Notifications.svelte";
  import { notifCenter } from "@/stores/notifications";

  let name: string;
  let email: string;
  let password: string;

  function submit() {
    ApiRoutes.register({ name, email, password })
      .then((_) => {
        location.replace("/login");
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
    >Name
    <input type="text" bind:value={name} required />
  </label>

  <label
    >Email
    <input type="email" bind:value={email} required />
  </label>

  <label
    >Password
    <input type="password" bind:value={password} required />
  </label>

  <button type="submit">Register</button>
</form>
