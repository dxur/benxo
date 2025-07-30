<script>
    import * as Sidebar from "$lib/components/ui/sidebar/index";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index";
    import { Separator } from "$lib/components/ui/separator/index";

    let { sidebar, children } = $props();
</script>

<Sidebar.Provider>
    {@render sidebar()}
    <Sidebar.Inset>
        <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
            <Sidebar.Trigger class="-ml-1" />
            <Separator
                orientation="vertical"
                class="mr-2 data-[orientation=vertical]:h-4"
            />
            <Breadcrumb.Root>
                <Breadcrumb.List>
                    <Breadcrumb.Item class="hidden md:block">
                        <Breadcrumb.Link href="#"
                            >Building Your Application</Breadcrumb.Link
                        >
                    </Breadcrumb.Item>
                    <Breadcrumb.Separator class="hidden md:block" />
                    <Breadcrumb.Item>
                        <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
                    </Breadcrumb.Item>
                </Breadcrumb.List>
            </Breadcrumb.Root>
        </header>
        <div class="flex flex-1 flex-col gap-4 p-4">
            {@render children?.()}
        </div>
    </Sidebar.Inset>
</Sidebar.Provider>
