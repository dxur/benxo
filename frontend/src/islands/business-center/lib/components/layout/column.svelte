<script>
    import { cn } from "$lib/utils";

    let props = $props();
    let { children, class: className, ...restProps } = props;
</script>

<div class={cn("flex flex-col gap-4", className)}>
    {@render children?.()}
</div>
