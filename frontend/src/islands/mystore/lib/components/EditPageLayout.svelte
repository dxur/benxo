<script lang="ts">
    import { Button } from "@/lib/components/ui/button/index.js";
    import { Badge } from "@/lib/components/ui/badge/index.js";
    import { Separator } from "@/lib/components/ui/separator/index.js";
    import type { Component, Snippet } from "svelte";
    import type { IconProps } from "@lucide/svelte";

    type HeaderAction = {
        label: string;
        icon?: Component;
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
        icon?: Component<IconProps, {}, "">;
    };

    type EditPageLayoutProps = {
        headerActions?: HeaderAction[];
        statusInfo?: StatusInfo;
        title: string;
        subtitle?: string;
        icon?: Component;
        sidebar?: Snippet;
        back_button?: Snippet;
        children?: Snippet;
    };

    let {
        headerActions = [],
        statusInfo,
        title,
        subtitle,
        icon: Icon,
        sidebar,
        back_button,
        children,
    }: EditPageLayoutProps = $props();
</script>

<div class="w-full bg-background">
    <div class="mx-auto max-w-7xl px-4 py-6">
        <!-- Page Header -->
        <header class="mb-8">
            <div
                class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between"
            >
                <div class="flex items-center gap-4">
                    {#if Icon}
                        <div
                            class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10 shrink-0"
                        >
                            <Icon class="h-6 w-6 text-primary" />
                        </div>
                    {/if}

                    <div class="min-w-0 flex-1">
                        <div class="flex items-center gap-3 flex-wrap">
                            <h1
                                class="text-2xl font-bold tracking-tight sm:text-3xl"
                            >
                                {title}
                            </h1>

                            {#if statusInfo}
                                <Badge
                                    variant={statusInfo.variant || "default"}
                                    class="gap-1"
                                >
                                    {#if statusInfo.icon}
                                        {@const Component = statusInfo.icon}
                                        <Component class="h-3 w-3" />
                                    {/if}
                                    {statusInfo.label}
                                </Badge>
                            {/if}
                        </div>

                        {#if subtitle}
                            <p
                                class="text-muted-foreground mt-1 text-sm sm:text-base"
                            >
                                {subtitle}
                            </p>
                        {/if}
                    </div>
                </div>

                <div class="flex items-center gap-2 shrink-0">
                    {@render back_button?.()}

                    {#if headerActions.length > 0}
                        <Separator orientation="vertical" class="h-6" />
                        <div class="flex items-center gap-2 flex-wrap">
                            {#each headerActions as action}
                                <Button
                                    variant={action.variant || "default"}
                                    size="sm"
                                    onclick={action.onclick}
                                    disabled={action.disabled}
                                    href={action.href}
                                    class="gap-2"
                                >
                                    {#if action.loading}
                                        <div
                                            class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
                                        ></div>
                                    {:else if action.icon}
                                        {@const Component = action.icon}
                                        <Component class="h-4 w-4" />
                                    {/if}
                                    {action.label}
                                </Button>
                            {/each}
                        </div>
                    {/if}
                </div>
            </div>
        </header>

        <!-- Content Grid -->
        <div class="grid grid-cols-1 gap-6 lg:grid-cols-5">
            <!-- Main Content -->
            <main class="lg:col-span-3">
                {@render children?.()}
            </main>

            <!-- Sidebar -->
            <aside class="lg:col-span-2">
                <div class="sticky top-6 space-y-6">
                    {@render sidebar?.()}
                </div>
            </aside>
        </div>
    </div>
</div>
