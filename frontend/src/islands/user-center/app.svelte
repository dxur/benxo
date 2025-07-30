<script lang="ts">
    import { list, switch_business } from "@bindings/BusinessRoutes";
    import type { BusinessView } from "@bindings/BusinessView";

    let businesses: BusinessView[] | undefined = $state(undefined);
    $effect(() => {
        list().then((res) => {
            businesses = res.businesses;
        });
    });

    function switch_biz(business_id: string) {
        switch_business({ business_id }).then(() => {
            console.log(`switched to biz ${business_id}`);
            location.href = "/business-center";
        });
    }
</script>

{#if businesses}
    <ul>
        {#each businesses as business}
            <li>
                <a href={undefined} onclick={() => switch_biz(business.id)}>
                    {business.id}
                </a>
                <span>{business.name}</span>
                <span>{business.description}</span>
                <span>{business.plan_type}</span>
                <span>{business.plan_expires_at}</span>
            </li>
        {/each}
    </ul>
{/if}
