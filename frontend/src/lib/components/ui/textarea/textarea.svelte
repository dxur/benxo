<script lang="ts">
	import {
		cn,
		type WithElementRef,
		type WithoutChildren,
	} from "$lib/utils.js";
	import type { HTMLTextareaAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		errors,
		...restProps
	}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> & {
		errors?: string[];
	} = $props();
</script>

<div class="space-y-1">
	<textarea
		bind:this={ref}
		aria-invalid={!!errors}
		data-slot="textarea"
		class={cn(
			"border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 field-sizing-content shadow-xs flex min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base outline-none transition-[color,box-shadow] focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
			className,
		)}
		bind:value
		{...restProps}
	></textarea>
	{#each errors || [] as error}
		<p class="text-sm text-destructive mt-1">{error}</p>
	{/each}
</div>
