<script lang="ts">
    import * as Collapsible from "@/lib/components/ui/collapsible/index.js";
    import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
    import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
    import LayoutDahboardIcon from "@lucide/svelte/icons/layout-dashboard";
    import type { Component } from "svelte";
    import {
        active,
        link,
        useNavigate,
    } from "@dvcol/svelte-simple-router/router";

    const { resolve, push, replace, back, forward, go } = useNavigate();

    let {
        items,
    }: {
        items: {
            title: string;
            path?: string;
            icon?: Component;
            isActive?: boolean;
            items?: {
                title: string;
                path: string;
            }[];
        }[];
    } = $props();

    let openItemTitle: string | null = $state(
        items.find((item) => item.isActive)?.title || null,
    );

    function toggleItem(title: string, path?: string) {
        openItemTitle = openItemTitle === title ? null : title;
        if (path) {
            // push({ path });
        }
    }
</script>

<Sidebar.Group>
    <Sidebar.GroupLabel>
        <LayoutDahboardIcon class="size-4 mr-2" />
        Platform
    </Sidebar.GroupLabel>
    <Sidebar.Menu>
        {#each items as item (item.title)}
            <Collapsible.Root
                open={openItemTitle === item.title}
                class="group/collapsible"
            >
                {#snippet child({ props })}
                    <Sidebar.MenuItem {...props}>
                        <Collapsible.Trigger>
                            {#snippet child({ props })}
                                <Sidebar.MenuButton
                                    {...props}
                                    tooltipContent={item.title}
                                    onclick={() =>
                                        toggleItem(item.title, item.path)}
                                    class={openItemTitle === item.title
                                        ? "bg-sidebar-accent text-sidebar-accent-foreground"
                                        : ""}
                                >
                                    {#if item.icon}
                                        <item.icon />
                                    {/if}
                                    <span>{item.title}</span>
                                    <ChevronRightIcon
                                        class="ml-auto transition-transform duration-300 ease-in-out group-data-[state=open]/collapsible:rotate-90"
                                    />
                                </Sidebar.MenuButton>
                            {/snippet}
                        </Collapsible.Trigger>
                        <Collapsible.Content
                            class="overflow-hidden data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down"
                        >
                            <Sidebar.MenuSub
                                class="transition-all duration-300 ease-in-out"
                            >
                                {#each item.items ?? [] as subItem (subItem.title)}
                                    <Sidebar.MenuSubItem>
                                        <Sidebar.MenuSubButton>
                                            {#snippet child({ props })}
                                                <a
                                                    use:link
                                                    use:active
                                                    href={subItem.path}
                                                    {...props}
                                                >
                                                    <span>{subItem.title}</span>
                                                </a>
                                            {/snippet}
                                        </Sidebar.MenuSubButton>
                                    </Sidebar.MenuSubItem>
                                {/each}
                            </Sidebar.MenuSub>
                        </Collapsible.Content>
                    </Sidebar.MenuItem>
                {/snippet}
            </Collapsible.Root>
        {/each}
    </Sidebar.Menu>
</Sidebar.Group>
