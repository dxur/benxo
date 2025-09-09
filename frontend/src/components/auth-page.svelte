<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  import { auth, me } from "@bindings/UserRoutes";
  import { navigate } from "astro:transitions/client";
  // import { ModeWatcher } from "mode-watcher";
  import { untrack } from "svelte";

  let email = $state("");
  let email_otp = $state("");
  let phone = $state("");
  let phone_otp = $state("");
  let first_name = $state("");
  let last_name = $state("");
  let username = $state("");
  let password = $state("");

  let step = $state<"login" | "signup" | "emailOtp" | "phoneOtp">("login");
  let error = $state<string | null>(null);

  $effect(() => {
    step;
    untrack(() => {
      error = null;
    });
  });

  $effect(() => {
    me().then(() => {
      console.error("Logout first");
      navigate("/user-center/");
    });
  });

  function login(e: Event) {
    e.preventDefault();
    auth({ Login: { email, password } })
      .then(() => navigate("/user-center/"))
      .catch((err) => {
        error = err?.message || "Something went wrong while logging in.";
      });
  }

  function signup(e: Event) {
    e.preventDefault();
    auth({ SignupEmail: { email } })
      .then(() => {
        step = "emailOtp";
      })
      .catch((err) => {
        error = err?.message || "Something went wrong while logging in.";
      });
  }

  function signupEmailOtp(e: Event) {
    e.preventDefault();
    auth({ SignupPhone: { otp: email_otp, phone } })
      .then(() => {
        step = "phoneOtp";
      })
      .catch((err) => {
        error = err?.message || "Something went wrong while logging in.";
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
      .then(() => navigate("/user-center/"))
      .catch((err) => {
        error = err?.message || "Something went wrong while logging in.";
      });
  }
</script>

{#snippet errorComponent()}
  {#if error}
    <div
      class="rounded-md bg-red-50 p-3 text-sm text-red-600 border border-red-200"
    >
      {error}
    </div>
  {/if}
{/snippet}

<!-- <ModeWatcher /> -->

<div class="flex flex-col gap-6">
  <Card.Root class="overflow-hidden p-0">
    <Card.Content class="grid p-0 md:grid-cols-2">
      {#if step === "login"}
        <form class="p-6 md:p-8 flex flex-col gap-3" onsubmit={login}>
          <div class="text-center">
            <h1 class="text-2xl font-bold">Welcome back</h1>
            <p class="text-muted-foreground">Login to your account</p>
          </div>

          <div class="grid gap-3">
            <Label for="email">Email</Label>
            <Input
              id="email"
              type="email"
              bind:value={email}
              placeholder="m@example.com"
              required
            />
          </div>

          <div class="grid gap-3">
            <Label for="password">Password</Label>
            <Input
              id="password"
              type="password"
              bind:value={password}
              placeholder="••••••••"
              autocomplete="on"
              required
            />
          </div>

          {@render errorComponent()}

          <Button type="submit" class="w-full">Login</Button>

          <p class="text-sm text-center">
            Don’t have an account? <button
              onclick={() => (step = "signup")}
              class="underline underline-offset-4">Sign up</button
            >
          </p>
        </form>
      {:else if step === "signup"}
        <form class="p-6 md:p-8 flex flex-col gap-3" onsubmit={signup}>
          <h1 class="text-2xl font-bold text-center">Create your account</h1>

          <div class="grid gap-3">
            <Label>Email</Label>
            <Input
              type="email"
              bind:value={email}
              placeholder="m@example.com"
              required
            />
          </div>

          <div class="grid gap-3">
            <Label>Phone</Label>
            <Input
              type="tel"
              bind:value={phone}
              placeholder="+213 ..."
              required
            />
          </div>

          <div class="grid gap-3">
            <Label>First Name</Label>
            <Input
              type="text"
              bind:value={first_name}
              placeholder="John"
              required
            />
          </div>

          <div class="grid gap-3">
            <Label>Last Name</Label>
            <Input
              type="text"
              bind:value={last_name}
              placeholder="Walker"
              required
            />
          </div>

          <div class="grid gap-3">
            <Label>Username</Label>
            <Input
              type="text"
              bind:value={username}
              placeholder="dragonSlayer"
              required
            />
          </div>

          <div class="grid gap-3">
            <Label>Password</Label>
            <Input
              type="password"
              bind:value={password}
              autocomplete="on"
              required
            />
          </div>

          {@render errorComponent()}

          <Button type="submit" class="w-full">Next</Button>
          <p class="text-sm text-center">
            Have an account? <button
              onclick={() => (step = "login")}
              class="underline underline-offset-4">Login</button
            >
          </p>
        </form>
      {:else if step === "emailOtp"}
        <form class="p-6 md:p-8 flex flex-col gap-6" onsubmit={signupEmailOtp}>
          <h1 class="text-2xl font-bold text-center">Verify your email</h1>
          <Input
            type="text"
            bind:value={email_otp}
            placeholder="Enter email OTP"
            required
          />

          {@render errorComponent()}

          <Button type="submit" class="w-full">Next</Button>
        </form>
      {:else if step === "phoneOtp"}
        <form class="p-6 md:p-8 flex flex-col gap-6" onsubmit={signupPhoneOtp}>
          <h1 class="text-2xl font-bold text-center">Verify your phone</h1>
          <Input
            type="text"
            bind:value={phone_otp}
            placeholder="Enter phone OTP"
            required
          />

          {@render errorComponent()}

          <Button type="submit" class="w-full">Finish Signup</Button>
        </form>
      {/if}

      <div class="bg-muted relative hidden md:block">
        <img
          src="/placeholder.svg"
          alt="placeholder"
          class="absolute inset-0 h-full w-full object-cover dark:brightness-[0.2] dark:grayscale"
        />
      </div>
    </Card.Content>
  </Card.Root>

  <div class="text-muted-foreground text-center text-xs">
    By continuing, you agree to our
    <a href="##" class="underline underline-offset-4">Terms of Service</a>
    and <a href="##" class="underline underline-offset-4">Privacy Policy</a>.
  </div>
</div>
