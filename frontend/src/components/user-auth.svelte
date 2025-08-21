<script lang="ts">
  import { me } from "@bindings/UserRoutes";
  import { userStore } from "@/lib/stores/user";
  import { navigate } from "astro:transitions/client";

  let { children } = $props();

  $effect(() => {
    me()
      .then((res) => {
        console.log("Welcome back");
        userStore.set(res);
      })
      .catch((_) => {
        console.error("Unautherized");
        navigate("/auth/");
      });
  });
</script>

{#if $userStore}
  {@render children?.()}
{/if}
