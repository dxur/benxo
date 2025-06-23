<script lang="ts">
  import * as ApiRoutes from "@bindings/ApiRoutes";

  export let redirect: string = "/";
  export let isAuth: boolean = true;
  export let includeReturnUrl: boolean = false;
  export let useReturnUrl: boolean = false;

  let show: boolean = false;

  function handleRedirect() {
    if (useReturnUrl) {
      const urlParams = new URLSearchParams(window.location.search);
      const returnUrl = urlParams.get("returnUrl");
      location.replace(returnUrl || redirect);
    } else {
      let url = redirect;
      if (includeReturnUrl) {
        url += `?returnUrl=${encodeURIComponent(location.pathname + location.search + location.hash)}`;
      }
      location.replace(url);
    }
  }

  $: {
    ApiRoutes.auth()
      .then(() => (isAuth ? (show = true) : handleRedirect()))
      .catch(() => (!isAuth ? (show = true) : handleRedirect()));
  }
</script>

{#if show}
  <slot />
{/if}
