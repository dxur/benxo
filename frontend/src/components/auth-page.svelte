<script lang="ts">
  import { auth, me } from "@bindings/UserRoutes";

  let email = $state("");
  let email_otp = $state("");
  let phone = $state("");
  let phone_otp = $state("");
  let first_name = $state("");
  let last_name = $state("");
  let username = $state("");
  let password = $state("");

  let step = $state<"login" | "signup" | "emailOtp" | "phoneOtp">("login");

  $effect(() => {
    me().then(() => {
      console.error("Logout first");
      location.href = "/user-center/";
    });
  });

  function login(e: Event) {
    e.preventDefault();
    auth({
      Login: {
        email,
        password,
      },
    })
      .then((_) => {
        location.href = "/user-center";
      })
      .catch((e) => {
        alert(e);
      });
  }

  function signup(e: Event) {
    e.preventDefault();
    auth({
      SignupEmail: {
        email,
      },
    })
      .then((_) => {
        step = "emailOtp";
      })
      .catch((e) => {
        alert(e);
      });
  }

  function signupEmailOtp(e: Event) {
    e.preventDefault();
    auth({
      SignupPhone: {
        otp: email_otp,
        phone,
      },
    })
      .then((_) => {
        step = "phoneOtp";
      })
      .catch((e) => {
        alert(e);
      });
  }

  function signupPhoneOtp(e: Event) {
    e.preventDefault();
    auth({
      SignupFinalize: {
        otp: phone_otp,
        first_name,
        last_name,
        username,
        password,
      },
    })
      .then((_) => {
        location.href = "/user-center";
      })
      .catch((e) => {
        alert(e);
      });
  }
</script>

<button
  onclick={() => {
    step = "login";
  }}>Login</button
>
<button
  onclick={() => {
    step = "signup";
  }}>Signup</button
>

{#if step === "login"}
  <form onsubmit={login}>
    <input bind:value={email} type="email" name="email" placeholder="Email" />
    <input
      bind:value={password}
      type="password"
      name="password"
      autocomplete="on"
      placeholder="Password"
    />
    <button>Login</button>
  </form>
{:else if step === "signup"}
  <form onsubmit={signup}>
    <input bind:value={email} type="email" name="email" placeholder="Email" />
    <input bind:value={phone} type="tel" name="phone" placeholder="+213 ..." />
    <input
      bind:value={first_name}
      type="text"
      name="first_name"
      placeholder="John"
    />
    <input
      bind:value={last_name}
      type="text"
      name="last_name"
      placeholder="Walker"
    />
    <input
      bind:value={username}
      type="text"
      name="username"
      placeholder="dragonSlayer"
    />
    <input
      bind:value={password}
      type="password"
      name="password"
      autocomplete="on"
      placeholder="Password"
    />
    <button>Next</button>
  </form>
{:else if step === "emailOtp"}
  <form onsubmit={signupEmailOtp}>
    <input
      bind:value={email_otp}
      type="text"
      name="Email Otp"
      placeholder="xxxxxx"
    />
    <button>Next</button>
  </form>
{:else if step === "phoneOtp"}
  <form onsubmit={signupPhoneOtp}>
    <input
      bind:value={phone_otp}
      type="text"
      name="Phone Otp"
      placeholder="xxxxxx"
    />
    <button>Signup</button>
  </form>
{/if}
