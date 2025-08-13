<script>
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index";
    import { useRoute } from "@dvcol/svelte-simple-router/router";
    import { useLink } from "@dvcol/svelte-simple-router";

    const { location } = $derived(useRoute());

    let items = $state([]);
    let current = $state("Loading...");
    $effect(() => {
        current = location?.name || current;
        let stack = [];
        let route = location?.meta?.parent;
        while (route) {
            stack.push({ name: route.name, path: route.path });
            route = route.meta?.parent;
        }
        items = stack.reverse();
    });
</script>

<Breadcrumb.Root>
    <Breadcrumb.List>
        {#each items as item}
            <Breadcrumb.Item>
                <Breadcrumb.Link href={item.path} {@attach useLink()}
                    >{item.name}</Breadcrumb.Link
                >
            </Breadcrumb.Item>
            <Breadcrumb.Separator />
        {/each}
        <Breadcrumb.Item>
            <Breadcrumb.Page>{current}</Breadcrumb.Page>
        </Breadcrumb.Item>
    </Breadcrumb.List>
</Breadcrumb.Root>
