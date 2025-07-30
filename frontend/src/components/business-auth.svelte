<script lang="ts">
    import { current } from "@bindings/BusinessRoutes";

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
                location.href = "/user-center";
            });
    });
</script>

{#if biz}
    {@render children?.(biz)}
{/if}
