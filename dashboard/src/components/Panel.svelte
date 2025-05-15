<script lang="ts">\n    export let children: any;\n</script>\n\n<div data-role="panel">\n    {#if children}\n        {children}\n    {/if}\n</div>
