<script lang="ts">
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index.js";
    import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
    import { useSidebar } from "@/lib/components/ui/sidebar/index.js";
    import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
    import PlusIcon from "@lucide/svelte/icons/plus";
    import BuildingIcon from "@lucide/svelte/icons/building";
    import UserIcon from "@lucide/svelte/icons/user";

    let {
        businesses,
    }: {
        businesses: {
            name: string;
            logo: any;
            plan: string;
            industry: string;
        }[];
    } = $props();
    const sidebar = useSidebar();
    let activeBusiness = $state(businesses[0]);
</script>

<Sidebar.Menu>
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
                            <activeBusiness.logo class="size-4" />
                        </div>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-medium">
                                {activeBusiness.name}
                            </span>
                            <span
                                class="truncate text-xs text-muted-foreground"
                            >
                                {activeBusiness.plan} • {activeBusiness.industry}
                            </span>
                        </div>
                        <ChevronsUpDownIcon class="ml-auto" />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-(--bits-dropdown-menu-anchor-width) min-w-64 rounded-lg"
                align="start"
                side={sidebar.isMobile ? "bottom" : "right"}
                sideOffset={4}
            >
                <DropdownMenu.Label class="text-muted-foreground text-xs"
                    >Switch Business</DropdownMenu.Label
                >
                {#each businesses as business, index (business.name)}
                    <DropdownMenu.Item
                        onSelect={() => (activeBusiness = business)}
                        class="gap-2 p-2"
                    >
                        <div
                            class="flex size-6 items-center justify-center rounded-md border"
                        >
                            <business.logo class="size-3.5 shrink-0" />
                        </div>
                        <div class="flex flex-col">
                            <span class="font-medium">{business.name}</span>
                            <span class="text-xs text-muted-foreground"
                                >{business.industry}</span
                            >
                        </div>
                        <div class="ml-auto flex items-center gap-2">
                            <span
                                class="text-xs bg-primary/10 text-primary px-2 py-1 rounded-full"
                            >
                                {business.plan}
                            </span>
                            <DropdownMenu.Shortcut
                                >⌘{index + 1}</DropdownMenu.Shortcut
                            >
                        </div>
                    </DropdownMenu.Item>
                {/each}
                <DropdownMenu.Separator />
                <DropdownMenu.Item class="gap-2 p-2">
                    <a
                        href="/user-center"
                        class="flex flex-row items-center gap-2"
                    >
                        <div
                            class="flex size-6 items-center justify-center rounded-md border bg-transparent text-muted-foreground"
                        >
                            <UserIcon class="size-4" />
                        </div>
                        <div class="font-medium">User Center</div>
                    </a>
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </Sidebar.MenuItem>
</Sidebar.Menu>
