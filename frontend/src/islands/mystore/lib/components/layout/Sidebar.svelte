<script lang="ts">
    import { _ } from "svelte-i18n";
</script>

<aside>
    <h1>{$_("common.dashboard")}</h1>
    <nav>
        <slot />
    </nav>
</aside>
