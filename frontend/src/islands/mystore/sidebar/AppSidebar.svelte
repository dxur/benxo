<script lang="ts" module>
    import AudioWaveformIcon from "@lucide/svelte/icons/audio-waveform";
    import BookOpenIcon from "@lucide/svelte/icons/book-open";
    import BotIcon from "@lucide/svelte/icons/bot";
    import ChartPieIcon from "@lucide/svelte/icons/chart-pie";
    import CommandIcon from "@lucide/svelte/icons/command";
    import FrameIcon from "@lucide/svelte/icons/frame";
    import GalleryVerticalEndIcon from "@lucide/svelte/icons/gallery-vertical-end";
    import MapIcon from "@lucide/svelte/icons/map";
    import Settings2Icon from "@lucide/svelte/icons/settings-2";
    import SquareTerminalIcon from "@lucide/svelte/icons/square-terminal";
    import ShoppingBagIcon from "@lucide/svelte/icons/shopping-bag";
    import PackageIcon from "@lucide/svelte/icons/package";
    import ShoppingCartIcon from "@lucide/svelte/icons/shopping-cart";
    import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
    import UsersIcon from "@lucide/svelte/icons/users";
    import CreditCardIcon from "@lucide/svelte/icons/credit-card";
    import BarChartIcon from "@lucide/svelte/icons/bar-chart";
    import StoreIcon from "@lucide/svelte/icons/store";
    import TagIcon from "@lucide/svelte/icons/tag";
    import TruckIcon from "@lucide/svelte/icons/truck";

    const data = {
        user: {
            name: "John Doe",
            email: "john@example.com",
            avatar: "/avatars/john.jpg",
        },
        businesses: [
            {
                name: "Coffee Shop Co",
                logo: StoreIcon,
                plan: "Pro",
                industry: "Food & Beverage",
            },
            {
                name: "Tech Gadgets Inc",
                logo: CommandIcon,
                plan: "Enterprise",
                industry: "Electronics",
            },
            {
                name: "Fashion Boutique",
                logo: ShoppingBagIcon,
                plan: "Starter",
                industry: "Fashion",
            },
        ],
        navMain: [
            {
                title: "Dashboard",
                url: "/dashboard",
                icon: SquareTerminalIcon,
                isActive: true,
                items: [
                    {
                        title: "Overview",
                        url: "/dashboard/overview",
                    },
                    {
                        title: "Analytics",
                        url: "/dashboard/analytics",
                    },
                    {
                        title: "Reports",
                        url: "/dashboard/reports",
                    },
                ],
            },
            {
                title: "Products",
                url: "/products",
                icon: PackageIcon,
                items: [
                    {
                        title: "All Products",
                        url: "/products",
                    },
                    {
                        title: "Categories",
                        url: "/products/categories",
                    },
                    {
                        title: "Inventory",
                        url: "/products/inventory",
                    },
                    {
                        title: "Pricing",
                        url: "/products/pricing",
                    },
                ],
            },
            {
                title: "Orders",
                url: "/orders",
                icon: ShoppingCartIcon,
                items: [
                    {
                        title: "All Orders",
                        url: "/orders",
                    },
                    {
                        title: "Pending",
                        url: "/orders/pending",
                    },
                    {
                        title: "Completed",
                        url: "/orders/completed",
                    },
                    {
                        title: "Refunds",
                        url: "/orders/refunds",
                    },
                ],
            },
            {
                title: "Customers",
                url: "/customers",
                icon: UsersIcon,
                items: [
                    {
                        title: "All Customers",
                        url: "/customers",
                    },
                    {
                        title: "Segments",
                        url: "/customers/segments",
                    },
                    {
                        title: "Reviews",
                        url: "/customers/reviews",
                    },
                    {
                        title: "Support",
                        url: "/customers/support",
                    },
                ],
            },
            {
                title: "Marketing",
                url: "/marketing",
                icon: TrendingUpIcon,
                items: [
                    {
                        title: "Campaigns",
                        url: "/marketing/campaigns",
                    },
                    {
                        title: "Email Marketing",
                        url: "/marketing/email",
                    },
                    {
                        title: "Promotions",
                        url: "/marketing/promotions",
                    },
                    {
                        title: "SEO",
                        url: "/marketing/seo",
                    },
                ],
            },
            {
                title: "Analytics",
                url: "/analytics",
                icon: BarChartIcon,
                items: [
                    {
                        title: "Sales Analytics",
                        url: "/analytics/sales",
                    },
                    {
                        title: "Conversions",
                        url: "/analytics/conversions",
                    },
                    {
                        title: "Traffic",
                        url: "/analytics/traffic",
                    },
                    {
                        title: "Performance",
                        url: "/analytics/performance",
                    },
                ],
            },
            {
                title: "Settings",
                url: "/settings",
                icon: Settings2Icon,
                items: [
                    {
                        title: "General",
                        url: "/settings/general",
                    },
                    {
                        title: "Payment Methods",
                        url: "/settings/payments",
                    },
                    {
                        title: "Shipping",
                        url: "/settings/shipping",
                    },
                    {
                        title: "Taxes",
                        url: "/settings/taxes",
                    },
                    {
                        title: "Integrations",
                        url: "/settings/integrations",
                    },
                ],
            },
        ],
        quickActions: [
            {
                name: "Add Product",
                url: "/products/new",
                icon: PackageIcon,
            },
            {
                name: "View Orders",
                url: "/orders",
                icon: ShoppingCartIcon,
            },
            {
                name: "Customer Analytics",
                url: "/analytics/customers",
                icon: UsersIcon,
            },
            {
                name: "Payment Settings",
                url: "/settings/payments",
                icon: CreditCardIcon,
            },
        ],
    };
</script>

<script lang="ts">
    import NavMain from "./NavMain.svelte";
    import NavQuickActions from "./NavQuickActions.svelte";
    import type { ComponentProps } from "svelte";
    import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
    import BusinessSwitcher from "./BusinessSwitcher.svelte";
    import NavUser from "./NavUser.svelte";

    let {
        ref = $bindable(null),
        collapsible = "icon",
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root {collapsible} {...restProps}>
    <Sidebar.Header>
        <BusinessSwitcher businesses={data.businesses} />
    </Sidebar.Header>
    <Sidebar.Content>
        <NavMain items={data.navMain} />
        <NavQuickActions actions={data.quickActions} />
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavUser user={data.user} />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
