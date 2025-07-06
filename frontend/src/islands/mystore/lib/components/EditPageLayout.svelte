<script lang="ts">
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import { Skeleton } from "@/lib/components/ui/skeleton/index.js";
    import { Separator } from "@/lib/components/ui/separator/index.js";
    import * as Breadcrumb from "@/lib/components/ui/breadcrumb/index.js";
    import { link } from "@dvcol/svelte-simple-router";
    import type { ComponentType } from "svelte";
    import { type LoadingStatus } from "@/components/StatusBoundary.svelte";
    import StatusBoundary from "@/components/StatusBoundary.svelte";

    type BreadcrumbItem = {
        label: string;
        href: string;
    };

    type HeaderAction = {
        label: string;
        icon?: ComponentType;
        variant?:
            | "default"
            | "outline"
            | "secondary"
            | "ghost"
            | "link"
            | "destructive";
        onclick?: () => void;
        disabled?: boolean;
        loading?: boolean;
        href?: string;
    };

    type StatusInfo = {
        label: string;
        variant?: "default" | "secondary" | "outline" | "destructive";
        icon?: ComponentType;
    };

    type EditPageLayoutProps = {
        breadcrumbs?: BreadcrumbItem[];
        headerActions?: HeaderAction[];
        statusInfo?: StatusInfo;
        title: string;
        subtitle?: string;
        icon?: ComponentType;
    };

    let {
        headerActions = [],
        statusInfo,
        title,
        subtitle,
        icon: Icon,
    }: EditPageLayoutProps = $props();
</script>

<div class="flex flex-col px-6 min-h-screen bg-background">
    <!-- Main Content -->
    <div class="flex flex-1">
        <!-- Main Content Area -->
        <main class="flex-1 overflow-hidden">
            <div class="container py-6">
                <!-- Page Header -->
                <div class="mb-8">
                    <div class="flex items-center gap-4 mb-4">
                        {#if Icon}
                            <div
                                class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10"
                            >
                                <Icon class="h-6 w-6 text-primary" />
                            </div>
                        {/if}
                        <div class="flex-1">
                            <div class="flex items-center gap-3">
                                <h1 class="text-3xl font-bold tracking-tight">
                                    {title}
                                </h1>

                                {#if statusInfo}
                                    <Badge
                                        variant={statusInfo.variant ||
                                            "default"}
                                        class="gap-1"
                                    >
                                        {#if statusInfo.icon}
                                            <svelte:component
                                                this={statusInfo.icon}
                                                class="h-3 w-3"
                                            />
                                        {/if}
                                        {statusInfo.label}
                                    </Badge>
                                {/if}
                            </div>

                            {#if subtitle}
                                <p class="text-muted-foreground mt-1">
                                    {subtitle}
                                </p>
                            {/if}
                        </div>
                        <!-- Header Actions -->
                        {#if headerActions.length > 0}
                            <Separator orientation="vertical" class="h-6" />
                            <div class="flex items-center gap-2">
                                {#each headerActions as action}
                                    <Button
                                        variant={action.variant || "default"}
                                        size="sm"
                                        onclick={action.onclick}
                                        disabled={action.disabled}
                                        href={action.href}
                                        class="gap-2"
                                    >
                                        <!-- use:link={action.href ? {} : undefined} -->
                                        {#if action.loading}
                                            <div
                                                class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
                                            ></div>
                                        {:else if action.icon}
                                            <svelte:component
                                                this={action.icon}
                                                class="h-4 w-4"
                                            />
                                        {/if}
                                        {action.label}
                                    </Button>
                                {/each}
                            </div>
                        {/if}
                    </div>
                </div>

                <!-- Content Grid -->
                <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                    <!-- Main Content -->
                    <div class="lg:col-span-3">
                        <slot name="content" />
                    </div>

                    <!-- Sidebar -->
                    <div class="lg:col-span-1">
                        <div class="sticky top-6">
                            <slot name="sidebar" />
                        </div>
                    </div>
                </div>
            </div>
        </main>
    </div>
</div>
