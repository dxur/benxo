<script>
  import { cn } from "$lib/utils";

  let props = $props();
  let { children, class: className, ...restProps } = props;
</script>

<div
  class={cn(
    "flex flex-col md:flex-row justify-between gap-4 w-full md:w-auto [&>*]:w-full md:[&>*]:w-auto lg:[&>*]:w-auto",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
</div>
