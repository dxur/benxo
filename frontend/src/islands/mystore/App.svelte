<script lang="ts">
  import type { RouterOptions } from "@dvcol/svelte-simple-router/models";
  import {
    RouterContext,
    RouterView,
  } from "@dvcol/svelte-simple-router/components";
  import routes from "./routes";
  import Sidebar from "./components/Sidebar.svelte";
  import Notifications from "@/components/Notifications.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { _, locale, locales, waitLocale } from "svelte-i18n";
  import "./i18n.ts";

  const languages = new Map([
    ["en", "English"],
    ["ar", "Arabic"],
    ["fr", "French"],
  ]);

  function logout() {
    ApiRoutes.logout().then((_) => {
      location.replace("/login");
    });
  }

  locale.subscribe((lang) => {
    if (!lang) return;

    localStorage.setItem("locale", lang);

    if (
      lang.startsWith("ar") ||
      lang.startsWith("fa") ||
      lang.startsWith("he")
    ) {
      document.documentElement.setAttribute("dir", "rtl");
    } else {
      document.documentElement.setAttribute("dir", "ltr");
    }
  });

  const options: RouterOptions = {
    routes,
    hash: true,
    priority: (a, b) => {
      return 0;
    },
  } as const;
</script>

{#await waitLocale() then}
  <RouterContext {options}>
    <Sidebar />
    <main>
      <header>
        <input type="text" placeholder={$_("common.actions.search")} />
        <select bind:value={$locale}>
          {#each $locales as locale}}
            <option value={locale}>{languages.get(locale)}</option>
          {/each}
        </select>
        <a href={undefined} on:click={() => logout()}
          >{$_("common.actions.logout")}</a
        >
      </header>

      <div data-page>
        <Notifications />
        <section data-page>
          <RouterView />
        </section>
      </div>
    </main>
  </RouterContext>
{/await}

<style>
  select {
    width: fit-content;
  }

  .account-menu {
    position: absolute;
    right: 0;
    top: 100%;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    display: none;
  }
  .account-menu.show {
    display: block;
  }
</style>
