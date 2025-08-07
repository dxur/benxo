<script lang="ts">
    import { list, switch_business, create } from "@bindings/BusinessRoutes";
    import type { BusinessDto } from "@bindings/BusinessDto";

    let businesses: BusinessDto[] | undefined = $state(undefined);

    $effect(() => {
        listBusinesses();
    });

    function listBusinesses() {
        list().then((res) => {
            businesses = res.businesses;
        });
    }

    function switch_biz(business_id: string) {
        switch_business({ business_id }).then(() => {
            console.log(`switched to biz ${business_id}`);
            location.href = "/business-center";
        });
    }

    let business_name = $state("");

    function createBusiness(e: Event) {
        e.preventDefault();
        create({
            name: business_name,
            description: null,
        }).then((res) => {
            console.log(res);
            listBusinesses();
        });
    }
</script>

<form onsubmit={createBusiness}>
    <input
        bind:value={business_name}
        type="text"
        name="Name"
        placeholder="BizX"
    />
    <button>Create Business</button>
</form>

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
