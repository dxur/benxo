<script lang="ts">
    import { me } from "@bindings/UserRoutes";

    let { children } = $props();
    let user: any = $state(undefined);

    $effect(() => {
        me()
            .then((res) => {
                console.log("Welcome back");
                user = res;
            })
            .catch((_) => {
                console.error("Unautherized");
                location.href = "/auth";
            });
    });
</script>

{#if user}
    {@render children?.(user)}
{/if}
