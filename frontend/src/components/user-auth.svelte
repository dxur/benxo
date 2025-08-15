<script lang="ts">
  import { me } from "@bindings/UserRoutes";
  import { userStore } from "@/lib/stores/user";

  let { children } = $props();

  $effect(() => {
    me()
      .then((res) => {
        console.log("Welcome back");
        userStore.set(res);
      })
      .catch((_) => {
        console.error("Unautherized");
        location.href = "/auth/";
      });
  });
</script>

{#if $userStore}
  {@render children?.()}
{/if}
