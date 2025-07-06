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
    import BellIcon from "@lucide/svelte/icons/bell";
    import HelpCircleIcon from "@lucide/svelte/icons/help-circle";
    import LayersIcon from "@lucide/svelte/icons/layers";
    import LineChartIcon from "@lucide/svelte/icons/line-chart";
    import TargetIcon from "@lucide/svelte/icons/target";
    import MegaphoneIcon from "@lucide/svelte/icons/megaphone";
    import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
    import { sidebar as productsSidebar } from "../features/products/index";
    import { sidebar as ordersSidebar } from "../features/orders/index";

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
                title: "Business Overview",
                path: "/business",
                icon: SquareTerminalIcon,
                isActive: true,
                items: [
                    {
                        title: "Business Dashboard",
                        path: "/business/dashboard",
                    },
                    {
                        title: "All Stores Performance",
                        path: "/business/stores-overview",
                    },
                    {
                        title: "Cross-Store Analytics",
                        path: "/business/cross-analytics",
                    },
                    {
                        title: "Business Activity",
                        path: "/business/activity",
                    },
                ],
            },
            {
                title: "Stores",
                path: "/stores",
                icon: StoreIcon,
                items: [
                    {
                        title: "All Stores",
                        path: "/stores",
                    },
                    {
                        title: "Store Templates",
                        path: "/stores/templates",
                    },
                    {
                        title: "Regional Management",
                        path: "/stores/regional",
                    },
                    {
                        title: "Testing Stores",
                        path: "/stores/testing",
                    },
                    {
                        title: "Campaign Stores",
                        path: "/stores/campaigns",
                    },
                    {
                        title: "Themes & Design",
                        path: "/stores/themes",
                    },
                    {
                        title: "Domains & SEO",
                        path: "/stores/domains",
                    },
                    {
                        title: "Store Performance",
                        path: "/stores/analytics",
                    },
                ],
            },
            productsSidebar,
            ordersSidebar,
            {
                title: "Customer Intelligence",
                path: "/customers",
                icon: UsersIcon,
                items: [
                    {
                        title: "All Customers",
                        path: "/customers",
                    },
                    {
                        title: "Customer Segments",
                        path: "/customers/segments",
                    },
                    {
                        title: "Cross-Store Customers",
                        path: "/customers/cross-store",
                    },
                    {
                        title: "Customer Journey",
                        path: "/customers/journey",
                    },
                    {
                        title: "Customer Support",
                        path: "/customers/support",
                    },
                    {
                        title: "Reviews & Feedback",
                        path: "/customers/reviews",
                    },
                    {
                        title: "Loyalty & Retention",
                        path: "/customers/loyalty",
                    },
                ],
            },
            {
                title: "Marketing & Growth",
                path: "/marketing",
                icon: MegaphoneIcon,
                items: [
                    {
                        title: "Campaign Hub",
                        path: "/marketing/campaigns",
                    },
                    {
                        title: "A/B Testing",
                        path: "/marketing/ab-testing",
                    },
                    {
                        title: "Email Marketing",
                        path: "/marketing/email",
                    },
                    {
                        title: "Social Media Ads",
                        path: "/marketing/social",
                    },
                    {
                        title: "Google Ads",
                        path: "/marketing/google-ads",
                    },
                    {
                        title: "Promotions & Coupons",
                        path: "/marketing/promotions",
                    },
                    {
                        title: "Multi-Store Campaigns",
                        path: "/marketing/multi-store",
                    },
                    {
                        title: "SEO & Content",
                        path: "/marketing/seo",
                    },
                    {
                        title: "Conversion Tracking",
                        path: "/marketing/conversions",
                    },
                ],
            },
            {
                title: "Analytics & Insights",
                path: "/analytics",
                icon: LineChartIcon,
                items: [
                    {
                        title: "Business Overview",
                        path: "/analytics/business",
                    },
                    {
                        title: "Store Comparison",
                        path: "/analytics/store-comparison",
                    },
                    {
                        title: "Sales Analytics",
                        path: "/analytics/sales",
                    },
                    {
                        title: "Revenue Reports",
                        path: "/analytics/revenue",
                    },
                    {
                        title: "Product Performance",
                        path: "/analytics/products",
                    },
                    {
                        title: "Customer Analytics",
                        path: "/analytics/customers",
                    },
                    {
                        title: "Marketing ROI",
                        path: "/analytics/marketing-roi",
                    },
                    {
                        title: "Testing Results",
                        path: "/analytics/testing",
                    },
                    {
                        title: "Traffic & Behavior",
                        path: "/analytics/traffic",
                    },
                    {
                        title: "Conversion Funnels",
                        path: "/analytics/funnels",
                    },
                    {
                        title: "Custom Reports",
                        path: "/analytics/custom",
                    },
                ],
            },
            {
                title: "Financial Management",
                path: "/finance",
                icon: DollarSignIcon,
                items: [
                    {
                        title: "Revenue Overview",
                        path: "/finance/revenue",
                    },
                    {
                        title: "Payment Methods",
                        path: "/finance/payments",
                    },
                    {
                        title: "Transactions",
                        path: "/finance/transactions",
                    },
                    {
                        title: "Taxes & Compliance",
                        path: "/finance/taxes",
                    },
                    {
                        title: "Invoicing",
                        path: "/finance/invoicing",
                    },
                    {
                        title: "Payouts",
                        path: "/finance/payouts",
                    },
                ],
            },
            {
                title: "Team & Permissions",
                path: "/team",
                icon: UsersIcon,
                items: [
                    {
                        title: "Team Members",
                        path: "/team/members",
                    },
                    {
                        title: "Roles & Permissions",
                        path: "/team/roles",
                    },
                    {
                        title: "Store Access Control",
                        path: "/team/store-access",
                    },
                    {
                        title: "Activity Log",
                        path: "/team/activity",
                    },
                    {
                        title: "Invite Members",
                        path: "/team/invite",
                    },
                ],
            },
            {
                title: "Settings",
                path: "/settings",
                icon: Settings2Icon,
                items: [
                    {
                        title: "General Settings",
                        path: "/settings/general",
                    },
                    {
                        title: "Business Profile",
                        path: "/settings/business",
                    },
                    {
                        title: "Shipping & Delivery",
                        path: "/settings/shipping",
                    },
                    {
                        title: "Notifications",
                        path: "/settings/notifications",
                    },
                    {
                        title: "Security",
                        path: "/settings/security",
                    },
                    {
                        title: "Billing & Plans",
                        path: "/settings/billing",
                    },
                ],
            },
        ],

        quickActions: [
            {
                name: "Add Product",
                path: "/products/create",
                icon: PackageIcon,
            },
            {
                name: "View Orders",
                path: "/orders",
                icon: ShoppingCartIcon,
            },
            {
                name: "Store Analytics",
                path: "/stores/analytics",
                icon: StoreIcon,
            },
            {
                name: "Sales Report",
                path: "/analytics/sales",
                icon: LineChartIcon,
            },
            {
                name: "Customer Support",
                path: "/customers/support",
                icon: HeartHandshakeIcon,
            },
            {
                name: "Marketing Campaign",
                path: "/marketing/campaigns/new",
                icon: TargetIcon,
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
    <!-- <Sidebar.Footer>
        <NavUser user={data.user} />
    </Sidebar.Footer> -->
    <Sidebar.Rail />
</Sidebar.Root>
