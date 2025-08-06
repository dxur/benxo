<script>
    import * as Card from "$lib/components/ui/card/index";
    import { Textarea } from "$lib/components/ui/textarea/index";
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Description</Card.Title>
        <Card.Description
            >Product description that customers will see.</Card.Description
        >
    </Card.Header>
    <Card.Content>
        <Textarea></Textarea>
    </Card.Content>
</Card.Root>
