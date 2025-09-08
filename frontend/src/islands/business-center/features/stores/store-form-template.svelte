<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Button } from "$lib/components/ui/button/index";
    import * as Select from "$lib/components/ui/select/index";
    import { Input } from "$lib/components/ui/input/index";
    import * as Tabs from "$lib/components/ui/tabs/index";
    import {
        SquarePenIcon,
        PlusIcon,
        TrashIcon,
        CopyIcon,
        RotateCcwIcon,
    } from "lucide-svelte";
    import { onMount } from "svelte";
    import { EditorView } from "codemirror";
    import { basicSetup } from "codemirror";
    import { liquid } from "@codemirror/lang-liquid";
    import { oneDark } from "@codemirror/theme-one-dark";
    import type { Form } from "../../lib/utils/form";
    import type { StoreSchema } from "./service";

    let {
        form = $bindable(),
    }: {
        form: Form<typeof StoreSchema>;
    } = $props();

    // Template editor state
    let isTemplateEditorOpen = $state(false);
    let selectedTemplate = $state<
        | "homepage"
        | "product_page"
        | "collection_page"
        | "cart_page"
        | "shop_page"
    >("homepage");
    let selectedSnippet = $state<string>("");
    let selectedCustomPage = $state<string>("");
    let newCustomPageName = $state("");
    let newSnippetName = $state("");

    // CodeMirror editor instances
    let templateEditorElement = $state<HTMLElement>();
    let snippetEditorElement = $state<HTMLElement>();
    let customPageEditorElement = $state<HTMLElement>();
    let templateEditor: EditorView | null = null;
    let snippetEditor: EditorView | null = null;
    let customPageEditor: EditorView | null = null;

    // Template options
    const templateOptions = [
        {
            value: "homepage",
            label: "Homepage Template",
            key: "homepage_template",
        },
        {
            value: "product_page",
            label: "Product Page Template",
            key: "product_page_template",
        },
        {
            value: "collection_page",
            label: "Collection Page Template",
            key: "collection_page_template",
        },
        {
            value: "cart_page",
            label: "Cart Page Template",
            key: "cart_page_template",
        },
        {
            value: "shop_page",
            label: "Shop Page Template",
            key: "shop_page_template",
        },
        {
            value: "not_found_page",
            label: "404 Not Found Template",
            key: "not_found_page_template",
        },
    ];

    // CodeMirror configuration
    const editorConfig = [
        basicSetup,
        liquid(),
        EditorView.theme({
            "&": { height: "400px" },
            ".cm-scroller": {
                fontFamily:
                    "ui-monospace, SFMono-Regular, 'SF Mono', monospace",
            },
            ".cm-focused": { outline: "none" },
        }),
        EditorView.updateListener.of((update) => {
            if (update.docChanged) {
                // Handle content changes here if needed
            }
        }),
    ];

    function toggleTemplateEditor() {
        isTemplateEditorOpen = !isTemplateEditorOpen;
        if (isTemplateEditorOpen) {
            // Initialize editors after the DOM is updated
            setTimeout(initializeEditors, 0);
        } else {
            // Clean up editors when closing
            destroyEditors();
        }
    }

    function destroyEditors() {
        if (templateEditor) {
            templateEditor.destroy();
            templateEditor = null;
        }
        if (snippetEditor) {
            snippetEditor.destroy();
            snippetEditor = null;
        }
        if (customPageEditor) {
            customPageEditor.destroy();
            customPageEditor = null;
        }
    }

    function initializeEditors() {
        // Initialize template editor
        if (templateEditorElement && !templateEditor) {
            templateEditor = new EditorView({
                doc: getCurrentTemplate(),
                extensions: [
                    ...editorConfig,
                    oneDark,
                    EditorView.updateListener.of((update) => {
                        if (update.docChanged) {
                            updateCurrentTemplate(update.state.doc.toString());
                        }
                    }),
                ],
                parent: templateEditorElement,
            });
        }

        // Initialize snippet editor
        if (snippetEditorElement && !snippetEditor) {
            snippetEditor = new EditorView({
                doc: getCurrentSnippet(),
                extensions: [
                    ...editorConfig,
                    oneDark,
                    EditorView.updateListener.of((update) => {
                        if (update.docChanged) {
                            updateCurrentSnippet(update.state.doc.toString());
                        }
                    }),
                ],
                parent: snippetEditorElement,
            });
        }

        // Initialize custom page editor
        if (customPageEditorElement && !customPageEditor) {
            customPageEditor = new EditorView({
                doc: getCurrentCustomPage(),
                extensions: [
                    ...editorConfig,
                    oneDark,
                    EditorView.updateListener.of((update) => {
                        if (update.docChanged) {
                            updateCurrentCustomPage(
                                update.state.doc.toString(),
                            );
                        }
                    }),
                ],
                parent: customPageEditorElement,
            });
        }
    }

    function getCurrentTemplate(): string {
        const templateKey = templateOptions.find(
            (t) => t.value === selectedTemplate,
        )?.key;
        if (templateKey) {
            return (
                (form[templateKey as keyof typeof form]?.value as string) || ""
            );
        }
        return "";
    }

    function updateCurrentTemplate(value: string) {
        const templateKey = templateOptions.find(
            (t) => t.value === selectedTemplate,
        )?.key;
        if (templateKey && form[templateKey as keyof typeof form]) {
            (form[templateKey as keyof typeof form] as any).value = value;
        }
    }

    function getCurrentSnippet() {
        if (!selectedSnippet) return "";
        return (form.snippets.value as any)?.[selectedSnippet] || "";
    }

    function updateCurrentSnippet(value: string) {
        if (!selectedSnippet) return;
        if (!form.snippets.value) {
            form.snippets.value = {};
        }
        (form.snippets.value as any)[selectedSnippet] = value;
    }

    function getCurrentCustomPage() {
        if (!selectedCustomPage) return "";
        return (form.custom_pages.value as any)?.[selectedCustomPage] || "";
    }

    function updateCurrentCustomPage(value: string) {
        if (!selectedCustomPage) return;
        if (!form.custom_pages.value) {
            form.custom_pages.value = {};
        }
        (form.custom_pages.value as any)[selectedCustomPage] = value;
    }

    function updateTemplateEditor() {
        if (templateEditor) {
            const currentContent = getCurrentTemplate();
            if (templateEditor.state.doc.toString() !== currentContent) {
                templateEditor.dispatch({
                    changes: {
                        from: 0,
                        to: templateEditor.state.doc.length,
                        insert: currentContent,
                    },
                });
            }
        }
    }

    function updateSnippetEditor() {
        if (snippetEditor) {
            const currentContent = getCurrentSnippet();
            if (snippetEditor.state.doc.toString() !== currentContent) {
                snippetEditor.dispatch({
                    changes: {
                        from: 0,
                        to: snippetEditor.state.doc.length,
                        insert: currentContent,
                    },
                });
            }
        }
    }

    function updateCustomPageEditor() {
        if (customPageEditor) {
            const currentContent = getCurrentCustomPage();
            if (customPageEditor.state.doc.toString() !== currentContent) {
                customPageEditor.dispatch({
                    changes: {
                        from: 0,
                        to: customPageEditor.state.doc.length,
                        insert: currentContent,
                    },
                });
            }
        }
    }

    function reset(type: "template" | "snippet" | "page") {
        if (type === "template") {
            const templateKey = templateOptions.find(
                (t) => t.value === selectedTemplate,
            )?.key;
            if (templateKey) {
                (form[templateKey as keyof typeof form] as any).value = "";
                updateTemplateEditor();
            }
        } else if (type === "snippet" && selectedSnippet) {
            if (form.snippets.value) {
                (form.snippets.value as any)[selectedSnippet] = "";
                updateSnippetEditor();
            }
        } else if (type === "page" && selectedCustomPage) {
            if (form.custom_pages.value) {
                (form.custom_pages.value as any)[selectedCustomPage] = "";
                updateCustomPageEditor();
            }
        }
    }

    function addCustomPage() {
        if (!newCustomPageName.trim()) return;
        if (!form.custom_pages.value) {
            form.custom_pages.value = {};
        }
        (form.custom_pages.value as any)[newCustomPageName.trim()] = "";
        selectedCustomPage = newCustomPageName.trim();
        newCustomPageName = "";
        setTimeout(updateCustomPageEditor, 0);
    }

    function deleteCustomPage(pageName: string) {
        if (!form.custom_pages.value) return;
        delete (form.custom_pages.value as any)[pageName];
        if (selectedCustomPage === pageName) {
            selectedCustomPage = "";
            updateCustomPageEditor();
        }
    }

    function addSnippet() {
        if (!newSnippetName.trim()) return;
        if (!form.snippets.value) {
            form.snippets.value = {};
        }
        (form.snippets.value as any)[newSnippetName.trim()] = "";
        selectedSnippet = newSnippetName.trim();
        newSnippetName = "";
        setTimeout(updateSnippetEditor, 0);
    }

    function deleteSnippet(snippetName: string) {
        if (!form.snippets.value) return;
        delete (form.snippets.value as any)[snippetName];
        if (selectedSnippet === snippetName) {
            selectedSnippet = "";
            updateSnippetEditor();
        }
    }

    function copyTemplate() {
        const content = getCurrentTemplate();
        navigator.clipboard.writeText(content);
    }

    function getSnippetsList() {
        return Object.keys(form.snippets.value || {});
    }

    function getCustomPagesList() {
        return Object.keys(form.custom_pages.value || {});
    }

    // Watch for template selection changes
    $effect(() => {
        if (selectedTemplate && isTemplateEditorOpen) {
            updateTemplateEditor();
        }
    });

    // Watch for snippet selection changes
    $effect(() => {
        if (selectedSnippet && isTemplateEditorOpen) {
            updateSnippetEditor();
        }
    });

    // Watch for custom page selection changes
    $effect(() => {
        if (selectedCustomPage && isTemplateEditorOpen) {
            updateCustomPageEditor();
        }
    });

    onMount(() => {
        return () => {
            // Cleanup editors on component destroy
            destroyEditors();
        };
    });
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Template Editor</Card.Title>
        <Card.Description>
            Customize your store templates, snippets, and custom pages.
        </Card.Description>
    </Card.Header>
    <Card.Content class="space-y-6">
        <div class="flex flex-col sm:flex-row gap-3">
            <Button onclick={toggleTemplateEditor} class="flex-1">
                <SquarePenIcon class="w-4 h-4 mr-2" />
                {isTemplateEditorOpen ? "Close" : "Open"} Template Editor
            </Button>
        </div>

        {#if isTemplateEditorOpen}
            <div class="border rounded-lg bg-card">
                <Tabs.Root value="templates" class="w-full">
                    <div class="border-b">
                        <Tabs.List class="grid w-full grid-cols-3 h-12">
                            <Tabs.Trigger value="templates" class="text-sm"
                                >Templates</Tabs.Trigger
                            >
                            <Tabs.Trigger value="snippets" class="text-sm"
                                >Snippets</Tabs.Trigger
                            >
                            <Tabs.Trigger value="custom-pages" class="text-sm"
                                >Custom Pages</Tabs.Trigger
                            >
                        </Tabs.List>
                    </div>

                    <!-- Templates Tab -->
                    <Tabs.Content value="templates" class="p-6 space-y-4">
                        <div
                            class="flex flex-col lg:flex-row lg:items-center gap-4"
                        >
                            <div class="flex-1 min-w-0">
                                <Select.Root
                                    type="single"
                                    bind:value={selectedTemplate}
                                >
                                    <Select.Trigger class="w-full mt-1">
                                        {templateOptions.find(
                                            (t) => t.value === selectedTemplate,
                                        )?.label || "Select Template"}
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each templateOptions as template}
                                            <Select.Item value={template.value}
                                                >{template.label}</Select.Item
                                            >
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <div class="flex flex-wrap gap-2">
                                <Button
                                    variant="outline"
                                    onclick={() => reset("template")}
                                >
                                    <RotateCcwIcon />
                                </Button>
                                <Button
                                    variant="outline"
                                    onclick={copyTemplate}
                                >
                                    <CopyIcon />
                                </Button>
                            </div>
                        </div>

                        <div
                            bind:this={templateEditorElement}
                            class="flex flex-row w-full min-h-[400px]"
                        ></div>
                    </Tabs.Content>

                    <!-- Snippets Tab -->
                    <Tabs.Content value="snippets" class="p-6 space-y-4">
                        <div
                            class="flex flex-col lg:flex-row lg:items-center gap-4"
                        >
                            <div class="flex-1 min-w-0">
                                <Select.Root
                                    type="single"
                                    bind:value={selectedSnippet}
                                >
                                    <Select.Trigger class="w-full">
                                        {selectedSnippet || "Select Snippet"}
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each getSnippetsList() as snippet}
                                            <Select.Item value={snippet}>
                                                <div
                                                    class="flex items-center justify-between w-full"
                                                >
                                                    <span>{snippet}</span>
                                                    <Button
                                                        variant="ghost"
                                                        class="h-6 w-6 p-0"
                                                        onclick={(e) => {
                                                            e.stopPropagation();
                                                            deleteSnippet(
                                                                snippet,
                                                            );
                                                        }}
                                                    >
                                                        <TrashIcon
                                                            class="w-3 h-3"
                                                        />
                                                    </Button>
                                                </div>
                                            </Select.Item>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <div class="flex gap-2">
                                <Input
                                    bind:value={newSnippetName}
                                    placeholder="New snippet name"
                                    class="w-full lg:w-48"
                                />
                                <Button
                                    onclick={addSnippet}
                                    disabled={!newSnippetName.trim()}
                                >
                                    <PlusIcon />
                                </Button>
                            </div>
                        </div>

                        <div class="flex gap-2">
                            <Button
                                variant="outline"
                                onclick={() => reset("snippet")}
                                disabled={!selectedSnippet}
                            >
                                <RotateCcwIcon />
                            </Button>
                        </div>

                        <div
                            bind:this={snippetEditorElement}
                            class="flex flex-row w-full min-h-[400px]"
                        ></div>
                    </Tabs.Content>

                    <!-- Custom Pages Tab -->
                    <Tabs.Content value="custom-pages" class="p-6 space-y-4">
                        <div
                            class="flex flex-col lg:flex-row lg:items-center gap-4"
                        >
                            <div class="flex-1 min-w-0">
                                <Select.Root
                                    type="single"
                                    bind:value={selectedCustomPage}
                                >
                                    <Select.Trigger class="w-full">
                                        {selectedCustomPage ||
                                            "Select Custom Page"}
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each getCustomPagesList() as page}
                                            <Select.Item value={page}>
                                                <div
                                                    class="flex items-center justify-between w-full"
                                                >
                                                    <span>{page}</span>
                                                    <Button
                                                        variant="ghost"
                                                        class="h-6 w-6 p-0"
                                                        onclick={(e) => {
                                                            e.stopPropagation();
                                                            deleteCustomPage(
                                                                page,
                                                            );
                                                        }}
                                                    >
                                                        <TrashIcon
                                                            class="w-3 h-3"
                                                        />
                                                    </Button>
                                                </div>
                                            </Select.Item>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            </div>

                            <div class="flex gap-2">
                                <Input
                                    bind:value={newCustomPageName}
                                    placeholder="New page name"
                                    class="w-48"
                                />
                                <Button
                                    onclick={addCustomPage}
                                    disabled={!newCustomPageName.trim()}
                                >
                                    <PlusIcon />
                                </Button>
                            </div>
                        </div>

                        <div class="flex gap-2">
                            <Button
                                variant="outline"
                                onclick={() => reset("page")}
                                disabled={!selectedCustomPage}
                            >
                                <RotateCcwIcon />
                            </Button>
                        </div>

                        <div
                            bind:this={customPageEditorElement}
                            class="flex flex-row w-full min-h-[400px]"
                        ></div>
                    </Tabs.Content>
                </Tabs.Root>
            </div>
        {/if}
    </Card.Content>
</Card.Root>
