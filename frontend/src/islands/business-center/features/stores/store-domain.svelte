<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Label } from "$lib/components/ui/label/index";
    import { Input } from "@/lib/components/ui/input";
    import { Button } from "@/lib/components/ui/button";
    import { Badge } from "@/lib/components/ui/badge";
    import {
        createMutation,
        createQuery,
        queryOptions,
    } from "@tanstack/svelte-query";
    import { toast } from "svelte-sonner";
    import { useRoute } from "@dvcol/svelte-simple-router/router";
    import {
        DomainRegSchema,
        setStoreReg,
        generateSlugFromName,
        fetchStore,
        StoreSchema,
        getStoreReg,
    } from "./service";
    import {
        createForm,
        getChangedValues,
        getFormValues,
        type Form,
    } from "../../lib/utils/form";
    import { useState } from "../../lib/utils/utils.svelte";
    import {
        CheckIcon,
        ExternalLinkIcon,
        RefreshCwIcon,
        WandIcon,
        AlertCircleIcon,
        GlobeIcon,
    } from "@lucide/svelte";
    import type { StoreRegUpdate } from "@bindings/StoreRegUpdate";
    import type { StoreRegDto } from "@bindings/StoreRegDto";
    import { formatDateTime } from "../../lib/utils/fmt";
    import { single } from "@/lib/event";

    let {
        storeId,
        store = $bindable(),
    }: {
        storeId: string;
        store: Form<typeof StoreSchema>;
    } = $props();

    const storeQuery = createQuery(() => ({
        queryKey: ["store-reg", storeId],
        queryFn: () => getStoreReg(storeId),
        enabled: !!storeId,
    }));

    let { form } = $derived(
        useState(createForm(DomainRegSchema, storeQuery.data)),
    );

    const regMutation = createMutation(() => ({
        mutationKey: ["store-reg", storeId],
        mutationFn: (updateReq: StoreRegUpdate) =>
            setStoreReg(storeId, updateReq),
        onSuccess: (data: StoreRegDto) => {
            toast.success("Domain settings updated successfully");
            storeQuery.refetch();
        },
        onError: (error) => {
            toast.error("Error updating domain settings", {
                description: error.message,
            });
        },
    }));

    function generateSlug() {
        if (store.name.value) {
            const newSlug = generateSlugFromName(store.name.value);
            form.slug.value = newSlug;
        } else {
            toast.error("Store name is required to generate slug");
        }
    }

    async function handleSubmit() {
        try {
            const changed_values = getChangedValues(form);
            if (!changed_values) {
                toast.warning("No changes detected");
                return;
            }
            const values = getFormValues(form);
            regMutation.mutate(values);
        } catch (e) {
            console.error("Validation Error", e);
            const errors = e as [string, string[]][];
            errors.forEach(([field, errors]) => {
                toast.error(`Error in ${field}`, {
                    description: errors.join("\n"),
                });
            });
        }
    }

    const slugPreviewUrl = $derived.by(() => {
        if (form.slug.initialValue) {
            return `https://${form.slug.value}.${__APP_ENV__.STORE_SUFFIX}/`;
        }
        return "";
    });

    const domainPreviewUrl = $derived.by(() => {
        if (form.domain.initialValue) {
            return `https://${form.domain.value}`;
        }
        return "";
    });
</script>

<div class="space-y-6">
    <!-- Store Slug Card -->
    <Card.Root>
        <Card.Header>
            <Card.Title class="flex items-center gap-2">
                <GlobeIcon size={20} />
                Store URL & Slug
            </Card.Title>
            <Card.Description>
                Set a unique identifier for your store URL. This will be used in
                your store's web address.
            </Card.Description>
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="space-y-2">
                <div class="flex items-center justify-between">
                    <Label for="slug">Store Slug</Label>
                    <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onclick={single(generateSlug)}
                        disabled={!store.name.value}
                    >
                        <WandIcon size={14} />
                        Generate from name
                    </Button>
                </div>
                <div class="flex items-center space-x-2">
                    <Input
                        id="slug"
                        type="text"
                        placeholder="my-awesome-store"
                        bind:value={form.slug.value}
                        errors={form.slug.errors}
                    />
                    <span class="text-sm text-muted-foreground"
                        >.{__APP_ENV__.STORE_SUFFIX}</span
                    >
                </div>
                {#if slugPreviewUrl}
                    <div class="flex items-center gap-2 p-2">
                        <span class="text-sm text-muted-foreground m-0"
                            >Preview:</span
                        >
                        <a
                            href={slugPreviewUrl}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-sm text-blue-600 hover:underline flex items-center gap-1"
                        >
                            {slugPreviewUrl}
                            <ExternalLinkIcon size={12} />
                        </a>
                    </div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>

    <!-- Custom Domain Card -->
    <Card.Root>
        <Card.Header>
            <Card.Title>Custom Domain</Card.Title>
            <Card.Description>
                Connect your own domain to your store for a professional
                appearance.
            </Card.Description>
        </Card.Header>
        <Card.Content class="space-y-4">
            <div class="space-y-2">
                <Label for="custom_domain">Custom Domain (Optional)</Label>
                <Input
                    id="custom_domain"
                    type="text"
                    placeholder="www.yourstore.com"
                    bind:value={form.domain.value}
                    errors={form.domain.errors}
                />
                <p class="text-sm text-muted-foreground">
                    Ensure your domain is properly configured to point to our
                    servers.
                    <a href="#" class="text-blue-600 hover:underline">
                        Learn how to configure DNS
                    </a>
                </p>
                {#if domainPreviewUrl}
                    <div class="flex items-center gap-2 p-2 bg-m">
                        <span class="text-sm text-muted-foreground m-0"
                            >Preview:</span
                        >
                        <a
                            href={domainPreviewUrl}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-sm text-blue-600 hover:underline flex items-center gap-1"
                        >
                            {domainPreviewUrl}
                            <ExternalLinkIcon size={12} />
                        </a>
                    </div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>

    <!-- Current Registration Status -->
    {#if storeQuery.data}
        <Card.Root>
            <Card.Header>
                <Card.Title>Current Registration</Card.Title>
                <Card.Description>
                    Your current domain and slug configuration.
                </Card.Description>
            </Card.Header>
            <Card.Content class="space-y-3">
                <div class="flex items-center justify-between">
                    <span class="text-sm font-medium">Slug:</span>
                    <Badge variant="secondary">{storeQuery.data.slug}</Badge>
                </div>
                <div class="flex items-center justify-between">
                    <span class="text-sm font-medium">Custom Domain:</span>
                    {#if storeQuery.data.domain}
                        <Badge variant="outline">{storeQuery.data.domain}</Badge
                        >
                    {:else}
                        <Badge variant="outline">Not set</Badge>
                    {/if}
                </div>
                <div class="flex items-center justify-between">
                    <span class="text-sm font-medium">Last Updated:</span>
                    <span class="text-sm text-muted-foreground">
                        {formatDateTime(storeQuery.data.updated_at)}
                    </span>
                </div>
            </Card.Content>
        </Card.Root>
    {/if}

    <!-- Action Buttons -->
    <div class="flex items-center justify-end gap-2">
        <Button
            type="button"
            variant="outline"
            onclick={() => {
                // Reset form to initial values
                if (storeQuery.data) {
                    form.slug.value = storeQuery.data.slug;
                    form.domain.value = storeQuery.data.domain || "";
                }
            }}
        >
            <RefreshCwIcon size={16} />
            Reset
        </Button>
        <Button type="button" onclick={single(handleSubmit)}>
            {#if regMutation.isPending}
                <RefreshCwIcon size={16} class="animate-spin" />
                Updating...
            {:else}
                <CheckIcon size={16} />
                Update Domain Settings
            {/if}
        </Button>
    </div>
</div>
