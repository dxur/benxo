<script>
  import {
    RouterContext,
    RouterView,
  } from "@dvcol/svelte-simple-router/components";
  import SunIcon from "@lucide/svelte/icons/sun";
  import MoonIcon from "@lucide/svelte/icons/moon";
  import routes, { AppRoutes } from "./routes";
  import AppSidebar from "./sidebar/AppSidebar.svelte";
  import Notifications from "@/components/Notifications.svelte";
  import FilePicker from "./lib/services/file-picker/FilePicker.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { _, locale, locales, waitLocale } from "svelte-i18n";
  import "./i18n.ts";
  import { ModeWatcher } from "mode-watcher";
  import { toggleMode } from "mode-watcher";
  import { Button } from "@/lib/components/ui/button/index.ts";
  import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
  import * as Breadcrumb from "@/lib/components/ui/breadcrumb/index.js";
  import { Separator } from "@/lib/components/ui/separator/index.js";

  import { Globals, Locale, Willow } from "wx-svelte-core";

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

  const options = {
    routes,
    hash: true,
    priority: (a, b) => {
      return 0;
    },
  };
</script>

<ModeWatcher />
{#await waitLocale() then}
  <RouterContext {options}>
    <Notifications />
    <FilePicker />
    <Sidebar.Provider>
      <AppSidebar />
      <Sidebar.Inset>
        <header
          class="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex justify-between px-4 gap-2 h-16 shrink-0 items-center transition-[width,height] ease-linear"
        >
          <div class="flex items-center gap-2">
            <Sidebar.Trigger class="-ml-1" />
            <Separator
              orientation="vertical"
              class="mr-2 data-[orientation=vertical]:h-4"
            />
            <Breadcrumb.Root>
              <Breadcrumb.List>
                <Breadcrumb.Item>
                  <Breadcrumb.Page>Breadcrumb</Breadcrumb.Page>
                </Breadcrumb.Item>
              </Breadcrumb.List>
            </Breadcrumb.Root>
          </div>
          <Button onclick={toggleMode} variant="outline" size="icon">
            <SunIcon
              class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
            />
            <MoonIcon
              class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
            />
            <span class="sr-only">Toggle theme</span>
          </Button>
        </header>
        <RouterView />
      </Sidebar.Inset>
    </Sidebar.Provider>
  </RouterContext>
{/await}
