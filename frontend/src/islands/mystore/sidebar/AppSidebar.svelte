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
    import DollarSignIcon from "@lucide/svelte/icons/dollar-sign";
    import WarehouseIcon from "@lucide/svelte/icons/warehouse";
    import HeartHandshakeIcon from "@lucide/svelte/icons/heart-handshake";
    import ZapIcon from "@lucide/svelte/icons/zap";
    import RepeatIcon from "@lucide/svelte/icons/repeat";
    import BellIcon from "@lucide/svelte/icons/bell";
    import HelpCircleIcon from "@lucide/svelte/icons/help-circle";
    import LayersIcon from "@lucide/svelte/icons/layers";
    import LineChartIcon from "@lucide/svelte/icons/line-chart";
    import TargetIcon from "@lucide/svelte/icons/target";
    import MegaphoneIcon from "@lucide/svelte/icons/megaphone";
    import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
    import { sidebar as productsSidebar } from "../features/products/index";
    import { sidebar as ordersSidebar } from "../features/orders/index";
    import { sidebar as storesSidebar } from "../features/online-stores/index";
    import { sidebar as shippingSidebar } from "../features/shipping-and-payment/index";
    import { Routes as productsRoutes } from "../features/products/index";
    import { Routes as shippingRoutes } from "../features/shipping-and-payment/index";
    import { Routes as ordersRoutes } from "../features/orders/index";

    const data = {
        user: {
            name: "John Doe",
            email: "john@example.com",
            avatar: "/avatars/john.jpg",
        },

        businesses: [
            {
                id: "biz-1",
                name: "Coffee Shop Co",
                logo: StoreIcon,
                plan: "Pro",
                industry: "Food & Beverage",
                role: "Owner",
                stores: [
                    {
                        id: "store-1",
                        name: "Downtown Main",
                        type: "main",
                        region: "NYC",
                        status: "active",
                    },
                    {
                        id: "store-2",
                        name: "Brooklyn Branch",
                        type: "regional",
                        region: "NYC",
                        status: "active",
                    },
                    {
                        id: "store-3",
                        name: "Coffee Test Lab",
                        type: "testing",
                        region: "NYC",
                        status: "active",
                    },
                    {
                        id: "store-4",
                        name: "Seasonal Winter",
                        type: "seasonal",
                        region: "NYC",
                        status: "inactive",
                    },
                ],
            },
            {
                id: "biz-2",
                name: "Tech Gadgets Inc",
                logo: CommandIcon,
                plan: "Enterprise",
                industry: "Electronics",
                role: "Admin",
                stores: [
                    {
                        id: "store-5",
                        name: "US Main Store",
                        type: "main",
                        region: "US",
                        status: "active",
                    },
                    {
                        id: "store-6",
                        name: "EU Store",
                        type: "regional",
                        region: "EU",
                        status: "active",
                    },
                    {
                        id: "store-7",
                        name: "Mobile Testing",
                        type: "testing",
                        region: "US",
                        status: "active",
                    },
                    {
                        id: "store-8",
                        name: "B2B Portal",
                        type: "b2b",
                        region: "Global",
                        status: "active",
                    },
                    {
                        id: "store-9",
                        name: "Black Friday Store",
                        type: "campaign",
                        region: "US",
                        status: "inactive",
                    },
                ],
            },
            {
                id: "biz-3",
                name: "Fashion Boutique",
                logo: ShoppingBagIcon,
                plan: "Starter",
                industry: "Fashion",
                role: "Marketing Manager",
                stores: [
                    {
                        id: "store-10",
                        name: "Main Boutique",
                        type: "main",
                        region: "LA",
                        status: "active",
                    },
                    {
                        id: "store-11",
                        name: "Trend Testing",
                        type: "testing",
                        region: "LA",
                        status: "active",
                    },
                ],
            },
        ],
        currentBusiness: {
            id: "biz-1",
            name: "Coffee Shop Co",
            role: "Owner",
        },
        currentStore: {
            id: "store-1",
            name: "Downtown Main",
            type: "main",
            region: "NYC",
            business: "Coffee Shop Co",
            status: "active",
        },

        navMain: [
            {
                title: "Dashboard",
                path: "/dashboard",
                icon: SquareTerminalIcon,
            },
            productsSidebar,
            ordersSidebar,
            storesSidebar,
            shippingSidebar,
            {
                title: "Settings",
                path: "/settings/basic",
                icon: Settings2Icon,
                items: [
                    {
                        title: "Business Settings",
                        path: "/settings/business",
                    },
                    {
                        title: "Account Settings",
                        path: "/settings/account",
                    },
                    {
                        title: "Shipping Profiles",
                        path: "/settings/shipping",
                    },
                    {
                        title: "Notification Settings",
                        path: "/settings/notifications",
                    },
                    {
                        title: "Security Settings",
                        path: "/settings/security",
                    },
                    {
                        title: "Billing Settings",
                        path: "/settings/billing",
                    },
                ],
            },
        ],

        quickActions: [
            {
                name: "Add Product",
                path: productsRoutes.CREATE_PAGE.path,
                icon: PackageIcon,
            },
            {
                name: "Add Order",
                path: ordersRoutes.CREATE_PAGE.path,
                icon: ShoppingCartIcon,
            },
            {
                name: "Add Conversion",
                path: "/stores/conversions/create",
                icon: RepeatIcon,
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
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavQuickActions actions={data.quickActions} />
        <!-- <NavUser user={data.user} /> -->
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
