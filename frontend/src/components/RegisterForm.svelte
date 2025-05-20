<script lang="ts">
  import * as ApiRoutes from "@bindings/ApiRoutes";

  let name: string;
  let email: string;
  let password: string;

  function submit() {
    console.log("register");
    ApiRoutes.register({ name, email, password }).then((_) => {
      location.assign("/login");
    });
  }
</script>

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
