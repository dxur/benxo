<script lang="ts">
  import * as ApiRoutes from "@bindings/ApiRoutes";

  let email: string;
  let password: string;

  function submit() {
    console.log("register");
    ApiRoutes.login({ email, password });
  }
</script>

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
