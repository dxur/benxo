<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Button } from "$lib/components/ui/button/index";
    import * as Select from "$lib/components/ui/select/index";
    import { Label } from "$lib/components/ui/label/index";
    import { EditIcon } from "lucide-svelte";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";
    import { snakeToTitleCase } from "../../lib/utils/fmt";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof StoreSchema>;
    } = $props();

    function openTemplateEditor() {
        // Navigate to template editor or open modal
        console.log("Opening template editor...");
    }
</script>

<!-- Quick Settings -->
<Card.Root>
    <Card.Header>
        <Card.Title>Quick Settings</Card.Title>
        <Card.Description>
            Basic customization options for your selected theme.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <Label>Color Scheme</Label>
                <Select.Root
                    type="single"
                    bind:value={form.color_scheme.value!}
                >
                    <Select.Trigger class="w-full">
                        {snakeToTitleCase(form.color_scheme.value) ||
                            "Select Color Scheme"}
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Item value="light">Light</Select.Item>
                        <Select.Item value="dark">Dark</Select.Item>
                        <Select.Item value="auto">Auto (System)</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>
            <div>
                <Label>Header Style</Label>
                <Select.Root
                    type="single"
                    bind:value={form.header_style.value!}
                >
                    <Select.Trigger class="w-full"
                        >{snakeToTitleCase(form.header_style.value) ||
                            "Select Header Style"}</Select.Trigger
                    >
                    <Select.Content>
                        <Select.Item value="centered">Centered</Select.Item>
                        <Select.Item value="left">Left Aligned</Select.Item>
                        <Select.Item value="sticky">Sticky</Select.Item>
                    </Select.Content>
                </Select.Root>
            </div>
        </div>
    </Card.Content>
</Card.Root>

<!-- Advanced Customization -->
<Card.Root>
    <Card.Header>
        <Card.Title>Advanced Customization</Card.Title>
        <Card.Description>
            Access the full template editor for detailed customization.
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <div class="flex flex-col sm:flex-row gap-3">
            <Button onclick={openTemplateEditor} class="flex-1">
                <EditIcon class="w-4 h-4 mr-2" />
                Open Template Editor
            </Button>
            <!-- <Button variant="outline" class="flex-1">
                <Settings class="w-4 h-4 mr-2" />
                Theme Settings
            </Button> -->
        </div>
    </Card.Content>
</Card.Root>

<!-- Theme Selection -->
<!-- <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between">
        <div>
            <Card.Title>Current Theme</Card.Title>
            <Card.Description>
                Choose and customize your store's appearance.
            </Card.Description>
        </div>
        <Badge variant="secondary">Modern Commerce v2.1.0</Badge>
    </Card.Header>
    <Card.Content class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            {#each themes as theme}
                <button
                    type="button"
                    class="border rounded-lg p-3 cursor-pointer transition-colors {selectedTheme ===
                    theme.value
                        ? 'border-primary bg-primary/5'
                        : 'border-border hover:border-primary/50'}"
                    onclick={() => (selectedTheme = theme.value)}
                >
                    <div
                        class="aspect-video bg-muted rounded mb-2 flex items-center justify-center"
                    >
                        <Palette class="w-8 h-8 text-muted-foreground" />
                    </div>
                    <h4 class="font-medium text-sm">{theme.label}</h4>
                </button>
            {/each}
        </div>
    </Card.Content>
</Card.Root> -->
