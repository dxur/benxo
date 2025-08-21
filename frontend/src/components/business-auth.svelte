<script lang="ts">
    import { current } from "@bindings/BusinessRoutes";
    import { navigate } from "astro:transitions/client";

    let { children } = $props();
    let biz: any = $state(undefined);

    $effect(() => {
        current()
            .then((res) => {
                console.log("Welcome back");
                biz = res;
            })
            .catch((_) => {
                console.error("Unautherized");
                navigate("/user-center/");
            });
    });
</script>

{#if biz}
    {@render children?.(biz)}
{/if}
