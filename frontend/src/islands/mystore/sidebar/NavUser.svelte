<script lang="ts">
    import * as Avatar from "@/lib/components/ui/avatar/index.js";
    import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index.js";
    import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
    import { useSidebar } from "@/lib/components/ui/sidebar/index.js";
    import BadgeCheckIcon from "@lucide/svelte/icons/badge-check";
    import BellIcon from "@lucide/svelte/icons/bell";
    import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
    import CreditCardIcon from "@lucide/svelte/icons/credit-card";
    import LogOutIcon from "@lucide/svelte/icons/log-out";
    import SparklesIcon from "@lucide/svelte/icons/sparkles";
    import SettingsIcon from "@lucide/svelte/icons/settings";
    import HelpCircleIcon from "@lucide/svelte/icons/help-circle";
    import UserIcon from "@lucide/svelte/icons/user";
    import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";

    let { user }: { user: { name: string; email: string; avatar: string } } =
        $props();
    const sidebar = useSidebar();
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({ props })}
                    <Sidebar.MenuButton
                        size="lg"
                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        {...props}
                    >
                        <Avatar.Root class="size-8 rounded-lg">
                            <Avatar.Image
                                src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=32&h=32&fit=crop&crop=face"
                                alt={user.name}
                            />
                            <Avatar.Fallback class="rounded-lg font-semibold">
                                {user.name
                                    .split(" ")
                                    .map((n) => n[0])
                                    .join("")}
                            </Avatar.Fallback>
                        </Avatar.Root>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-medium">{user.name}</span
                            >
                            <span class="truncate text-xs text-muted-foreground"
                                >{user.email}</span
                            >
                        </div>
                        <ChevronsUpDownIcon class="ml-auto size-4" />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
                side={sidebar.isMobile ? "bottom" : "right"}
                align="end"
                sideOffset={4}
            >
                <DropdownMenu.Label class="p-0 font-normal">
                    <div
                        class="flex items-center gap-2 px-1 py-1.5 text-left text-sm"
                    >
                        <Avatar.Root class="size-8 rounded-lg">
                            <Avatar.Image src={user.avatar} alt={user.name} />
                            <Avatar.Fallback class="rounded-lg font-semibold">
                                {user.name
                                    .split(" ")
                                    .map((n) => n[0])
                                    .join("")}
                            </Avatar.Fallback>
                        </Avatar.Root>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-medium">{user.name}</span
                            >
                            <span class="truncate text-xs text-muted-foreground"
                                >{user.email}</span
                            >
                        </div>
                    </div>
                </DropdownMenu.Label>
                <DropdownMenu.Separator />
                <DropdownMenu.Group>
                    <DropdownMenu.Item>
                        <SparklesIcon class="size-4" />
                        Upgrade Plan
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <ShieldCheckIcon class="size-4" />
                        Business Verification
                    </DropdownMenu.Item>
                </DropdownMenu.Group>
                <DropdownMenu.Separator />
                <DropdownMenu.Group>
                    <DropdownMenu.Item>
                        <UserIcon class="size-4" />
                        Profile Settings
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <BadgeCheckIcon class="size-4" />
                        Account Security
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <CreditCardIcon class="size-4" />
                        Billing & Invoices
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <BellIcon class="size-4" />
                        Notifications
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <SettingsIcon class="size-4" />
                        Preferences
                    </DropdownMenu.Item>
                </DropdownMenu.Group>
                <DropdownMenu.Separator />
                <DropdownMenu.Group>
                    <DropdownMenu.Item>
                        <HelpCircleIcon class="size-4" />
                        Help & Support
                    </DropdownMenu.Item>
                </DropdownMenu.Group>
                <DropdownMenu.Separator />
                <DropdownMenu.Item
                    class="text-red-600 focus:text-red-600 focus:bg-red-50"
                >
                    <LogOutIcon class="size-4" />
                    Log out
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </Sidebar.MenuItem>
</Sidebar.Menu>
