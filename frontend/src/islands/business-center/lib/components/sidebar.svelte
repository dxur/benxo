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
    import { active, link } from "@dvcol/svelte-simple-router";
    import {
        ChevronsUpDownIcon,
        LogOut,
        GalleryVerticalEnd,
    } from "@lucide/svelte";

    let { items }: { items: SidebarItem[] } = $props();
    let sidebar = useSidebar();
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
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-medium">
                                activeTeam.name
                            </span>
                            <span class="truncate text-xs">activeTeam.plan</span
                            >
                        </div>
                        <ChevronsUpDownIcon />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
                align="start"
                side={sidebar.isMobile ? "bottom" : "right"}
                sideOffset={4}
            >
                <DropdownMenu.Label class="text-muted-foreground text-xs"
                    >Businesses</DropdownMenu.Label
                >
                <DropdownMenu.Item class="gap-2 p-2">
                    team.name
                </DropdownMenu.Item>
                <DropdownMenu.Separator />
                <DropdownMenu.Item class="gap-2 p-2">
                    <LogOut class="size-4" />
                    <a
                        href="/user-center"
                        class="text-muted-foreground font-medium"
                    >
                        User center
                    </a>
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
                    <a href={item.path} {...props} use:link use:active>
                        <item.icon />
                        <span>{item.name}</span>
                    </a>
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
