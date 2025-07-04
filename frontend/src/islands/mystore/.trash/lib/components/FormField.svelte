<script lang="ts">
    // The 'config' prop receives an object like:
    // { type: 'text', required: true, pattern: '...' }
    export let config: {
        type: "text" | "textarea" | "number" | "checkbox" | "select";
        required?: boolean;
        pattern?: string;
        step?: string;
        label: string;
    };

    console.log(config);

    export let value: any;
</script>

<fieldset>
    <label>
        {config.label}

        {#if config.type === "textarea"}
            <textarea rows="10" bind:value required={config.required}
            ></textarea>
        {:else if config.type === "checkbox"}
            <input
                type="checkbox"
                bind:checked={value}
                required={config.required}
            />
        {:else}
            <input
                type={config.type}
                bind:value
                required={config.required}
                pattern={config.pattern}
                step={config.step}
            />
        {/if}
    </label>
</fieldset>
