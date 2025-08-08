<script>
    import * as Sidebar from "$lib/components/ui/sidebar/index";
    import { Separator } from "$lib/components/ui/separator/index";
    import { Toaster } from "$lib/components/ui/sonner/index";

    let { sidebar, breadcrumb, children } = $props();
</script>

<Toaster />

<Sidebar.Provider>
    {@render sidebar()}
    <div class="bg-wrapper">
        <Sidebar.Inset class="">
            <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
                <Sidebar.Trigger class="-ml-1" />
                <Separator
                    orientation="vertical"
                    class="mr-2 data-[orientation=vertical]:h-4"
                />
                {@render breadcrumb()}
            </header>
            <div class="flex flex-1 flex-col gap-4 p-4">
                {@render children?.()}
            </div>
        </Sidebar.Inset>
    </div>
</Sidebar.Provider>
