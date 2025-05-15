<script lang="ts">
    import { notifCenter } from "../stores/notifications";
</script>

<div data-notif>
    {#each $notifCenter as { id, msgType, title, msg }}
        <div data-msg data-role={msgType}>
            <strong>{title}</strong>
            {msg}
            <button type="reset" on:click={() => notifCenter.removeNotif(id)}
                >×</button
            >
        </div>
    {/each}
</div>
