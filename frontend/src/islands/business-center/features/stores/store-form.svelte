<script lang="ts">
    import Group from "../../lib/components/layout/group.svelte";
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "$lib/components/ui/input/index";
    import * as Card from "$lib/components/ui/card/index";
    import * as Tabs from "$lib/components/ui/tabs/index";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";

    let {
        form = $bindable(),
        general,
        domain,
        template,
        extra,
        all,
        disabled,
    }: {
        form: Form<typeof StoreSchema>;
        general?: boolean;
        domain?: boolean;
        template?: boolean;
        extra?: boolean;
        all?: boolean;
        disabled?: boolean;
    } = $props();

    const tabs = [
        {
            value: "general",
            label: "General",
            snip: generalTab,
            vis: general,
        },
        { value: "domain", label: "Domain", snip: domainTab, vis: domain },
        {
            value: "template",
            label: "Template",
            snip: templateTab,
            vis: template,
        },
        { value: "extra", label: "Extra", snip: extraTab, vis: extra },
    ];
</script>

<Group class="max-w-7xl md:flex-col md:[&>*]:w-full lg:flex-row">
    <form class="flex-1">
        <Tabs.Root value="general">
            {#if all || tabs.some((tab) => tab.vis)}
                <Tabs.List class="w-full">
                    {#each tabs as { value, label, vis }}
                        {#if vis || all}
                            <Tabs.Trigger {value}>
                                {label}
                            </Tabs.Trigger>
                        {/if}
                    {/each}
                </Tabs.List>
            {/if}
            <fieldset {disabled}>
                {#each tabs as tab}
                    {#if tab.vis || all}
                        <Tabs.Content value={tab.value}>
                            {@render tab.snip?.()}
                        </Tabs.Content>
                    {/if}
                {/each}
            </fieldset>
        </Tabs.Root>
    </form>
    <div class="lg:max-w-sm space-y-6">
        <div class="lg:w-xs">
            {@render summary()}
        </div>
    </div>
</Group>

{#snippet generalTab()}
    <div class="tab-content">
        <Card.Root>
            <Card.Header>
                <Card.Title>Basic Information</Card.Title>
                <Card.Description>
                    Essential store details that customers will see.
                </Card.Description>
            </Card.Header>
            <Card.Content class="">
                <div>
                    <Label for="title">Store Name</Label>
                    <Input
                        id="title"
                        errors={form.name.errors}
                        bind:value={form.name.value}
                        placeholder="Enter store name"
                    />
                </div>
                <div>
                    <Label for="description">Description</Label>
                    <Input
                        id="description"
                        errors={form.description.errors}
                        bind:value={form.description.value}
                        placeholder="Enter store description"
                    />
                </div>
                <div>
                    <Label for="category">Category</Label>
                    <Input id="category" placeholder="e.g. Electronics" />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Contact & Location</Card.Title>
                <Card.Description>
                    How customers can reach or find your store.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="contact_email">Contact Email</Label>
                    <Input
                        id="contact_email"
                        type="email"
                        placeholder="e.g. info@store.com"
                    />
                </div>
                <div>
                    <Label for="contact_phone">Contact Phone</Label>
                    <Input
                        id="contact_phone"
                        type="tel"
                        placeholder="e.g. +1 555 123 4567"
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Location</Card.Title>
                <Card.Description
                    >Physical address for your store.</Card.Description
                >
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="address">Address</Label>
                    <Input
                        id="address"
                        placeholder="e.g. 123 Main St, City, Country"
                    />
                </div>
                <div>
                    <Label for="zip_code">Zip Code</Label>
                    <Input id="zip_code" placeholder="e.g. 12345" />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Social Media Links</Card.Title>
                <Card.Description>
                    Connect your store with social media platforms.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="facebook">Facebook</Label>
                    <Input
                        id="facebook"
                        placeholder="e.g. facebook.com/yourstore"
                    />
                </div>
                <div>
                    <Label for="instagram">Instagram</Label>
                    <Input
                        id="instagram"
                        placeholder="e.g. instagram.com/yourstore"
                    />
                </div>
                <div>
                    <Label for="twitter">Twitter</Label>
                    <Input
                        id="twitter"
                        placeholder="e.g. twitter.com/yourstore"
                    />
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}

{#snippet domainTab()}
    <div class="tab-content">
        <Card.Root>
            <Card.Header>
                <Card.Title>Domain Settings</Card.Title>
                <Card.Description>
                    Configure your store's web address and domain preferences.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="domain">Custom Domain</Label>
                    <Input
                        id="domain"
                        type="url"
                        placeholder="e.g. www.yourstore.com"
                    />
                </div>
                <div>
                    <Label for="subdomain">Subdomain</Label>
                    <Input
                        id="subdomain"
                        placeholder="e.g. yourstore.example.com"
                    />
                </div>
                <div>
                    <Label for="redirect_url">Redirect URL</Label>
                    <Input
                        id="redirect_url"
                        type="url"
                        placeholder="e.g. https://oldstore.com"
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>SSL & Security</Card.Title>
                <Card.Description>
                    Security settings for your domain and store.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="ssl_enabled">SSL Certificate</Label>
                    <Input
                        id="ssl_enabled"
                        placeholder="Automatically managed"
                        readonly
                    />
                </div>
                <div>
                    <Label for="force_https">Force HTTPS</Label>
                    <Input
                        id="force_https"
                        placeholder="Enabled by default"
                        readonly
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>DNS Settings</Card.Title>
                <Card.Description
                    >Advanced DNS configuration options.</Card.Description
                >
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="cname_record">CNAME Record</Label>
                    <Input
                        id="cname_record"
                        placeholder="e.g. shops.example.com"
                    />
                </div>
                <div>
                    <Label for="a_record">A Record</Label>
                    <Input id="a_record" placeholder="e.g. 192.168.1.1" />
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}

{#snippet templateTab()}
    <div class="tab-content">
        <Card.Root>
            <Card.Header>
                <Card.Title>Theme Selection</Card.Title>
                <Card.Description>
                    Choose and customize your store's appearance.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="theme">Current Theme</Label>
                    <Input
                        id="theme"
                        placeholder="e.g. Modern Commerce"
                        readonly
                    />
                </div>
                <div>
                    <Label for="theme_version">Theme Version</Label>
                    <Input
                        id="theme_version"
                        placeholder="e.g. 2.1.0"
                        readonly
                    />
                </div>
                <div>
                    <Label for="color_scheme">Color Scheme</Label>
                    <Input
                        id="color_scheme"
                        placeholder="e.g. Light, Dark, Auto"
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Layout Options</Card.Title>
                <Card.Description>
                    Customize your store's layout and structure.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="header_layout">Header Layout</Label>
                    <Input
                        id="header_layout"
                        placeholder="e.g. Centered, Left-aligned, Sticky"
                    />
                </div>
                <div>
                    <Label for="footer_layout">Footer Layout</Label>
                    <Input
                        id="footer_layout"
                        placeholder="e.g. Simple, Multi-column, Minimal"
                    />
                </div>
                <div>
                    <Label for="sidebar_position">Sidebar Position</Label>
                    <Input
                        id="sidebar_position"
                        placeholder="e.g. Left, Right, None"
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Custom Styling</Card.Title>
                <Card.Description>
                    Advanced customization options for your store design.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="custom_css">Custom CSS</Label>
                    <Input
                        id="custom_css"
                        placeholder="Add custom styles here..."
                    />
                </div>
                <div>
                    <Label for="font_family">Font Family</Label>
                    <Input
                        id="font_family"
                        placeholder="e.g. Inter, Roboto, Arial"
                    />
                </div>
                <div>
                    <Label for="logo_url">Logo URL</Label>
                    <Input
                        id="logo_url"
                        type="url"
                        placeholder="e.g. https://example.com/logo.png"
                    />
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}

{#snippet extraTab()}
    <div class="tab-content">
        <Card.Root>
            <Card.Header>
                <Card.Title>Analytics & Tracking</Card.Title>
                <Card.Description>
                    Set up tracking and analytics for your store.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="google_analytics">Google Analytics ID</Label>
                    <Input
                        id="google_analytics"
                        placeholder="e.g. GA-XXXXXXXXX-X"
                    />
                </div>
                <div>
                    <Label for="facebook_pixel">Facebook Pixel ID</Label>
                    <Input
                        id="facebook_pixel"
                        placeholder="e.g. 1234567890123456"
                    />
                </div>
                <div>
                    <Label for="gtm_container">Google Tag Manager</Label>
                    <Input id="gtm_container" placeholder="e.g. GTM-XXXXXXX" />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>SEO Settings</Card.Title>
                <Card.Description>
                    Optimize your store for search engines.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="meta_title">Meta Title</Label>
                    <Input
                        id="meta_title"
                        placeholder="e.g. Your Store - Quality Products Online"
                    />
                </div>
                <div>
                    <Label for="meta_description">Meta Description</Label>
                    <Input
                        id="meta_description"
                        placeholder="e.g. Discover amazing products at unbeatable prices..."
                    />
                </div>
                <div>
                    <Label for="meta_keywords">Meta Keywords</Label>
                    <Input
                        id="meta_keywords"
                        placeholder="e.g. store, products, shopping, online"
                    />
                </div>
                <div>
                    <Label for="robots_txt">Robots.txt</Label>
                    <Input
                        id="robots_txt"
                        placeholder="e.g. User-agent: * Allow: /"
                    />
                </div>
            </Card.Content>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Title>Advanced Settings</Card.Title>
                <Card.Description>
                    Additional configuration options for power users.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <div>
                    <Label for="custom_scripts">Custom Scripts</Label>
                    <Input
                        id="custom_scripts"
                        placeholder="Add custom JavaScript here..."
                    />
                </div>
                <div>
                    <Label for="webhook_url">Webhook URL</Label>
                    <Input
                        id="webhook_url"
                        type="url"
                        placeholder="e.g. https://api.example.com/webhook"
                    />
                </div>
                <div>
                    <Label for="api_key">API Key</Label>
                    <Input
                        id="api_key"
                        type="password"
                        autocomplete="on"
                        placeholder="Your API key"
                    />
                </div>
                <div>
                    <Label for="timezone">Timezone</Label>
                    <Input id="timezone" placeholder="e.g. America/New_York" />
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}

{#snippet summary()}
    <Card.Root>
        <Card.Header>
            <Card.Title>Quick Stats</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-3">
            <div class="flex justify-between text-sm">
                <span class="text-muted-foreground">Variants</span>
                <span class="font-medium"></span>
            </div>
            <div class="flex justify-between text-sm">
                <span class="text-muted-foreground">Price</span>
                <span class="font-medium"> </span>
            </div>
        </Card.Content>
    </Card.Root>
{/snippet}
