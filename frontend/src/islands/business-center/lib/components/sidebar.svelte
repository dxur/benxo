<script module lang="ts">
  import type { Component } from "svelte";
  export type SidebarItem = {
    id: string;
    name: string;
    path: string;
    icon: Component;
    active?: boolean;
  };
</script>

<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index";
  import { useSidebar } from "$lib/components/ui/sidebar/index.js";
  import { active, link, useLink } from "@dvcol/svelte-simple-router";
  import {
    ChevronsUpDownIcon,
    LogOut,
    GalleryVerticalEnd,
    ChevronLeftIcon,
    ChevronRightIcon,
  } from "@lucide/svelte";
  import { current } from "@bindings/BusinessRoutes";
  import type { BusinessDto } from "@bindings/BusinessDto";
  import { toast } from "svelte-sonner";

  let { items }: { items: SidebarItem[] } = $props();
  let sidebar = useSidebar();

  let currentBusiness = $state<BusinessDto | undefined>(undefined);

  $effect(() => {
    current()
      .then((res) => {
        currentBusiness = res;
      })
      .catch(() => {
        toast("Something went wrong");
      });
  });
</script>

{#snippet bizSwitcher()}
  <Sidebar.MenuItem>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Sidebar.MenuButton
            {...props}
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <div
              class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
            >
              <GalleryVerticalEnd class="size-4" />
            </div>
            <div class="grid flex-1 text-left text-sm leading-tight">
              {#if currentBusiness}
                <span class="truncate font-medium">
                  {currentBusiness.name}
                </span>
                <span class="truncate text-xs">{currentBusiness.plan_type}</span
                >
              {/if}
            </div>
            <ChevronRightIcon />
          </Sidebar.MenuButton>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content
        class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
        align="start"
        side={sidebar.isMobile ? "bottom" : "right"}
        sideOffset={4}
      >
        <DropdownMenu.Item class="gap-2 p-2">
          <LogOut class="size-4" />
          <a href="/user-center/" class="font-medium"> User center </a>
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Sidebar.MenuItem>
{/snippet}

{#snippet mainNav()}
  {#each items as item}
    <Sidebar.MenuItem>
      <Sidebar.MenuButton>
        {#snippet child({ props })}
          <button
            {@attach useLink({ path: item.path })}
            {...props}
            use:link
            use:active={{ path: item.path }}
          >
            <item.icon />
            <span>{item.name}</span>
          </button>
        {/snippet}
      </Sidebar.MenuButton>
    </Sidebar.MenuItem>
  {/each}
{/snippet}

<Sidebar.Root collapsible="icon">
  <Sidebar.Header>
    <Sidebar.Menu>
      {@render bizSwitcher()}
    </Sidebar.Menu>
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.Menu>
        {@render mainNav()}
      </Sidebar.Menu>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer></Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
