<script lang="ts">
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index.js";
    import { useSidebar } from "@/lib/components/ui/sidebar/context.svelte.js";
    import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
    import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
    import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
    import StarIcon from "@lucide/svelte/icons/star";
    import PinIcon from "@lucide/svelte/icons/pin";
    import ZapIcon from "@lucide/svelte/icons/zap";

    let {
        actions,
    }: {
        actions: {
            name: string;
            url: string;
            // This should be `Component` after @lucide/svelte updates types
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            icon: any;
        }[];
    } = $props();
    const sidebar = useSidebar();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
    <Sidebar.GroupLabel>
        <ZapIcon class="size-4 mr-2" />
        Quick Actions
    </Sidebar.GroupLabel>
    <Sidebar.Menu>
        {#each actions as action (action.name)}
            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    {#snippet child({ props })}
                        <a href={action.url} {...props}>
                            <action.icon />
                            <span>{action.name}</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        {/each}
        <!-- <Sidebar.MenuItem>
            <Sidebar.MenuButton class="text-sidebar-foreground/70">
                <EllipsisIcon class="text-sidebar-foreground/70" />
                <span>More</span>
            </Sidebar.MenuButton>
        </Sidebar.MenuItem> -->
    </Sidebar.Menu>
</Sidebar.Group>
