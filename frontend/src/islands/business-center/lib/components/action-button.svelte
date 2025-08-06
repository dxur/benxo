<script>
    import { Button } from "$lib/components/ui/button/index";

    let props = $props();
    let { children, ...restProps } = props;
</script>

<Button class="gap-2" {...restProps}>
    {@render children?.()}
</Button>
