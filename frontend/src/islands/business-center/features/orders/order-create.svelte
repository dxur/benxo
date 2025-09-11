<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index";
    import * as Card from "$lib/components/ui/card/index";
    import * as Select from "$lib/components/ui/select/index";
    import { Button } from "@/lib/components/ui/button";
    import { Input } from "@/lib/components/ui/input";
    import { Label } from "@/lib/components/ui/label";
    import { Textarea } from "@/lib/components/ui/textarea";
    import { Checkbox } from "@/lib/components/ui/checkbox";
    import Column from "../../lib/components/layout/column.svelte";
    import Group from "../../lib/components/layout/group.svelte";
    import SectionHeader from "../../lib/components/section-header.svelte";
    import ActionButton from "../../lib/components/action-button.svelte";
    import {
        PlusIcon,
        SendIcon,
        ShoppingCartIcon,
        TrashIcon,
        ArrowLeftIcon,
        UserIcon,
        MapPinIcon,
        CreditCardIcon,
        PackageIcon,
    } from "@lucide/svelte";

    import {
        createMutation,
        createQuery,
        getQueryClientContext,
    } from "@tanstack/svelte-query";
    import { useNavigate } from "@dvcol/svelte-simple-router/router";
    import { useState } from "../../lib/utils/utils.svelte";
    import { createForm, getFormValues, type Form } from "../../lib/utils/form";
    import {
        OrderCreateSchema,
        useOrderCreate,
        createDefaultShippingAddress,
        createDefaultOrderItem,
    } from "./service";
    import { Routes } from ".";
    import { toast } from "svelte-sonner";
    import { snakeToTitleCase } from "../../lib/utils/fmt";
    import type { OrderCreate } from "@bindings/OrderCreate";
    import { single } from "../../lib/utils/event";

    const { replace } = useNavigate();

    let { form } = $derived(
        useState(
            createForm(OrderCreateSchema, <OrderCreate>{
                customer_email: "",
                customer_name: "",
                customer_phone: "",
                items: [createDefaultOrderItem()],
                shipping_address: createDefaultShippingAddress(),
                billing_address: null,
                shipping_cost: "0.00",
                tax_amount: "0.00",
                currency: "USD",
                notes: null,
            }),
        ),
    );
    let useBillingAddress = $state(false);

    const queryContext = getQueryClientContext();

    const creationMutation = useOrderCreate(
        (data) => {
            toast.success("Order created successfully");
            queryContext.invalidateQueries({
                queryKey: ["orders"],
            });
            replace({
                path: Routes.DETAIL_PAGE.path.replace(":id", data.id),
            });
        },
        (e) => {
            toast.error("Error creating order", {
                description: e.message,
            });
        },
    );

    function handleCreate() {
        try {
            const values = getFormValues<typeof OrderCreateSchema>(form);

            if (!useBillingAddress) {
                values.billing_address = null;
            }

            creationMutation.mutate(values);
        } catch (e) {
            console.error("Validation Error", e);
            const errors = e as [string, string[]][];
            errors.forEach(([field, errors]) => {
                toast.error(`Error in field: ${snakeToTitleCase(field)}`, {
                    description: errors.join("\n"),
                });
            });
        }
    }

    function addOrderItem() {
        if (form.items.value) {
            form.items.value = [...form.items.value, createDefaultOrderItem()];
        }
    }

    function removeOrderItem(index: number) {
        if (form.items.value && form.items.value.length > 1) {
            form.items.value = form.items.value.filter((_, i) => i !== index);
        }
    }

    function copyShippingToBilling() {
        if (form.shipping_address.value) {
            form.billing_address.value = { ...form.shipping_address.value };
        }
    }

    function goBack() {
        replace({ path: Routes.LIST_PAGE.path });
    }
</script>

<Column class="[&>*]:w-full items-center">
    <Group>
        <div class="flex items-center gap-4">
            <!-- <Button variant="ghost" size="icon" onclick={goBack}>
                <ArrowLeftIcon />
            </Button> -->
            <SectionHeader
                icon={ShoppingCartIcon}
                title="Create new order"
                description="Fill in the details below to create a new order"
            />
        </div>
        <Group class="md:flex-row-reverse flex-wrap justify-start">
            <ActionButton
                onclick={handleCreate}
                disabled={creationMutation.isPending}
            >
                <SendIcon />
                {creationMutation.isPending ? "Creating..." : "Create Order"}
            </ActionButton>
        </Group>
    </Group>

    <Group class="max-w-4xl md:flex-col md:[&>*]:w-full lg:flex-row">
        <div class="flex-1">
            <Tabs.Root value="customer">
                <Tabs.List class="w-full">
                    <Tabs.Trigger value="customer">Customer</Tabs.Trigger>
                    <Tabs.Trigger value="items">Items</Tabs.Trigger>
                    <Tabs.Trigger value="shipping">Shipping</Tabs.Trigger>
                    <Tabs.Trigger value="billing">Billing</Tabs.Trigger>
                    <Tabs.Trigger value="summary">Summary</Tabs.Trigger>
                </Tabs.List>

                <fieldset>
                    <Tabs.Content value="customer" class="tab-content">
                        {@render customerTab()}
                    </Tabs.Content>
                    <Tabs.Content value="items" class="tab-content">
                        {@render itemsTab()}
                    </Tabs.Content>
                    <Tabs.Content value="shipping" class="tab-content">
                        {@render shippingTab()}
                    </Tabs.Content>
                    <Tabs.Content value="billing" class="tab-content">
                        {@render billingTab()}
                    </Tabs.Content>
                    <Tabs.Content value="summary" class="tab-content">
                        {@render summaryTab()}
                    </Tabs.Content>
                </fieldset>
            </Tabs.Root>
        </div></Group
    >
</Column>

{#snippet customerTab()}
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex items-center gap-2">
                <UserIcon class="w-5 h-5" />
                Customer Information
            </Card.Title>
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                    <Label for="customer_name">Customer Name *</Label>
                    <Input
                        id="customer_name"
                        bind:value={form.customer_name.value}
                        placeholder="John Doe"
                        required
                    />
                </div>
                <div class="space-y-2">
                    <Label for="customer_phone">Phone Number *</Label>
                    <Input
                        id="customer_phone"
                        bind:value={form.customer_phone.value}
                        placeholder="+1234567890"
                        required
                    />
                </div>
            </div>
            <div class="space-y-2">
                <Label for="customer_email">Email Address</Label>
                <Input
                    id="customer_email"
                    type="email"
                    bind:value={form.customer_email.value}
                    placeholder="john@example.com"
                    required
                />
            </div>
        </Card.Content>
    </Card.Root>
{/snippet}

{#snippet itemsTab()}
    <Card.Root>
        <Card.Header>
            <div class="flex items-center justify-between">
                <Card.Title class="flex items-center gap-2">
                    <PackageIcon class="w-5 h-5" />
                    Order Items
                </Card.Title>
                <Button onclick={single(addOrderItem)} size="sm">
                    <PlusIcon class="w-4 h-4 mr-2" />
                    Add Item
                </Button>
            </div>
        </Card.Header>
        <Card.Content class="space-y-4">
            {#each form.items.value || [] as item, index}
                <div class="border rounded-lg p-4 space-y-4">
                    <div class="flex items-center justify-between">
                        <span class="font-medium">Item {index + 1}</span>
                        {#if (form.items.value?.length || 0) > 1}
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={single(() => removeOrderItem(index))}
                            >
                                <TrashIcon class="w-4 h-4" />
                            </Button>
                        {/if}
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="space-y-2">
                            <Label>Product ID *</Label>
                            <Input
                                bind:value={item.product_id}
                                placeholder="product-id"
                                required
                            />
                        </div>
                        <div class="space-y-2">
                            <Label>Variant SKU *</Label>
                            <Input
                                bind:value={item.variant_sku}
                                placeholder="SKU123"
                                required
                            />
                        </div>
                        <div class="space-y-2">
                            <Label>Quantity *</Label>
                            <Input
                                type="number"
                                bind:value={item.quantity}
                                min="1"
                                required
                            />
                        </div>
                    </div>
                </div>
            {/each}
        </Card.Content>
    </Card.Root>
{/snippet}

{#snippet shippingTab()}
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex items-center gap-2">
                <MapPinIcon class="w-5 h-5" />
                Shipping Address
            </Card.Title>
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="space-y-2">
                <Label>Full Name *</Label>
                <Input
                    bind:value={form.shipping_address.value.full_name}
                    placeholder="John"
                    required
                />
            </div>
            <div class="space-y-2">
                <Label>Address Line 1 *</Label>
                <Input
                    bind:value={form.shipping_address.value.address_line_1}
                    placeholder="123 Main Street"
                    required
                />
            </div>
            <div class="space-y-2">
                <Label>Address Line 2</Label>
                <Input
                    bind:value={form.shipping_address.value.address_line_2}
                    placeholder="Apartment, suite, etc. (optional)"
                />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="space-y-2">
                    <Label>City *</Label>
                    <Input
                        bind:value={form.shipping_address.value.city}
                        placeholder="New York"
                        required
                    />
                </div>
                <div class="space-y-2">
                    <Label>State/Province</Label>
                    <Input
                        bind:value={form.shipping_address.value.state}
                        placeholder="NY"
                    />
                </div>
                <div class="space-y-2">
                    <Label>Postal Code *</Label>
                    <Input
                        bind:value={form.shipping_address.value.postal_code}
                        placeholder="10001"
                        required
                    />
                </div>
            </div>
            <div class="space-y-2">
                <Label>Country *</Label>
                <Input
                    bind:value={form.shipping_address.value.country}
                    placeholder="United States"
                    required
                />
            </div>
        </Card.Content>
    </Card.Root>
{/snippet}

{#snippet billingTab()}
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex items-center gap-2">
                <CreditCardIcon class="w-5 h-5" />
                Billing Address
            </Card.Title>
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="flex items-center space-x-2">
                <Checkbox
                    id="use_billing"
                    bind:checked={useBillingAddress}
                    onCheckedChange={(checked) => {
                        if (!checked && form.billing_address.value) {
                            form.billing_address.value = null;
                        } else if (checked && !form.billing_address.value) {
                            form.billing_address.value =
                                createDefaultShippingAddress();
                        }
                    }}
                />
                <Label for="use_billing">Use different billing address</Label>
            </div>

            {#if useBillingAddress}
                <div class="space-y-4 pt-4 border-t">
                    <Button
                        variant="outline"
                        size="sm"
                        onclick={single(copyShippingToBilling)}
                        type="button"
                    >
                        Copy from shipping address
                    </Button>

                    <div class="space-y-2">
                        <Label>First Name *</Label>
                        <Input
                            bind:value={form.billing_address.value!.full_name}
                            placeholder="John"
                            required
                        />
                    </div>
                    <div class="space-y-2">
                        <Label>Address Line 1 *</Label>
                        <Input
                            bind:value={
                                form.billing_address.value!.address_line_1
                            }
                            placeholder="123 Main Street"
                            required
                        />
                    </div>
                    <div class="space-y-2">
                        <Label>Address Line 2</Label>
                        <Input
                            bind:value={
                                form.billing_address.value!.address_line_2
                            }
                            placeholder="Apartment, suite, etc. (optional)"
                        />
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="space-y-2">
                            <Label>City *</Label>
                            <Input
                                bind:value={form.billing_address.value!.city}
                                placeholder="New York"
                                required
                            />
                        </div>
                        <div class="space-y-2">
                            <Label>State/Province</Label>
                            <Input
                                bind:value={form.billing_address.value!.state}
                                placeholder="NY"
                            />
                        </div>
                        <div class="space-y-2">
                            <Label>Postal Code *</Label>
                            <Input
                                bind:value={
                                    form.billing_address.value!.postal_code
                                }
                                placeholder="10001"
                                required
                            />
                        </div>
                    </div>
                    <div class="space-y-2">
                        <Label>Country *</Label>
                        <Input
                            bind:value={form.billing_address.value!.country}
                            placeholder="United States"
                            required
                        />
                    </div>
                </div>
            {/if}
        </Card.Content>
    </Card.Root>
{/snippet}

{#snippet summaryTab()}
    <div class="space-y-6">
        <Card.Root>
            <Card.Header>
                <Card.Title>Order Details</Card.Title>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <Label>Shipping Cost *</Label>
                        <Input
                            type="number"
                            step="0.01"
                            min="0"
                            bind:value={form.shipping_cost.value}
                            placeholder="0.00"
                            required
                        />
                    </div>
                    <div class="space-y-2">
                        <Label>Tax Amount *</Label>
                        <Input
                            type="number"
                            step="0.01"
                            min="0"
                            bind:value={form.tax_amount.value}
                            placeholder="0.00"
                            required
                        />
                    </div>
                </div>
                <div class="space-y-2">
                    <Label>Currency</Label>
                    <Select.Root
                        type="single"
                        bind:value={form.currency.value!}
                    >
                        <Select.Trigger>
                            {form.currency.value || "USD"}
                        </Select.Trigger>
                        <Select.Content>
                            <Select.Item value="USD"
                                >USD - US Dollar</Select.Item
                            >
                            <Select.Item value="EUR">EUR - Euro</Select.Item>
                            <Select.Item value="GBP"
                                >GBP - British Pound</Select.Item
                            >
                            <Select.Item value="CAD"
                                >CAD - Canadian Dollar</Select.Item
                            >
                            <Select.Item value="AUD"
                                >AUD - Australian Dollar</Select.Item
                            >
                        </Select.Content>
                    </Select.Root>
                </div>
                <div class="space-y-2">
                    <Label>Notes</Label>
                    <Textarea
                        bind:value={form.notes.value}
                        placeholder="Additional notes for this order..."
                        rows={3}
                    />
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{/snippet}
