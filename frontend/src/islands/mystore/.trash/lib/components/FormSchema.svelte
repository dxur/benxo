<script lang="ts">
    import type {
        FormSchema,
        PolicyContext,
        ActionConfig,
    } from "../lib/schema";
    import { registry } from "../lib/registry";
    import { onMount } from "svelte";

    export let schema: FormSchema;
    export let data: Record<string, any> = {};

    let formData = { ...data };
    let errors: Record<string, string> = {};
    let loading = false;

    // Debounced update handlers
    const debouncedHandlers = new Map<string, NodeJS.Timeout>();

    // Create policy context
    const createContext = (field?: any, group?: any): PolicyContext => ({
        model: formData,
        field,
        group,
        schema,
        ...resolveContext(),
    });

    // Resolve dynamic context
    const resolveContext = () => {
        if (typeof schema.context === "function") {
            return schema.context(formData);
        }
        return schema.context || {};
    };

    // Execute an action with proper debouncing/throttling
    const executeAction = async (
        actionConfig: ActionConfig,
        actionData: any,
        context: PolicyContext,
        actionKey: string,
    ) => {
        // Clear existing debounced handler
        const existingHandler = debouncedHandlers.get(actionKey);
        if (existingHandler) {
            clearTimeout(existingHandler);
        }

        const executeHandler = async () => {
            try {
                // Check policy if specified
                if (actionConfig.policy) {
                    const policyResult = registry.evaluatePolicy(
                        actionConfig.policy,
                        context,
                    );
                    if (!policyResult.allowed) {
                        console.warn(
                            `Action blocked by policy: ${policyResult.reason}`,
                        );
                        return;
                    }
                }

                // Validate if required
                if (actionConfig.validate) {
                    const isValid = await validateForm();
                    if (!isValid) return;
                }

                loading = true;
                await actionConfig.handler(actionData, context);
            } catch (error) {
                console.error(`Action ${actionKey} failed:`, error);
            } finally {
                loading = false;
            }
        };

        // Apply debouncing if specified
        if (actionConfig.debounce) {
            const timeoutId = setTimeout(executeHandler, actionConfig.debounce);
            debouncedHandlers.set(actionKey, timeoutId);
        } else {
            await executeHandler();
        }
    };

    // Handle field updates
    const handleFieldUpdate = async (field: any, value: any) => {
        formData[field.key] = value;
        formData = { ...formData }; // Trigger reactivity

        // Clear field error
        delete errors[field.key];
        errors = { ...errors };

        const context = createContext(field);

        // Execute field-level update action
        if (field.actions?.onUpdate) {
            await executeAction(
                field.actions.onUpdate,
                { field: field.key, value, formData },
                context,
                `field-${field.key}-update`,
            );
        }

        // Execute form-level update action
        if (schema.actions?.onUpdate) {
            await executeAction(
                schema.actions.onUpdate,
                { field: field.key, value, formData },
                context,
                "form-update",
            );
        }
    };

    // Validate form
    const validateForm = async (): Promise<boolean> => {
        const newErrors: Record<string, string> = {};
        let isValid = true;

        // Validate each visible field
        for (const column of schema.columns) {
            for (const group of column) {
                const groupContext = createContext(null, group);
                const groupVisible = registry.checkPolicies(
                    group.policies,
                    groupContext,
                );

                if (!groupVisible.allowed) continue;

                for (const field of group.fields) {
                    const fieldContext = createContext(field, group);
                    const fieldVisible = registry.checkPolicies(
                        field.policies,
                        fieldContext,
                    );

                    if (!fieldVisible.allowed) continue;

                    // Execute validation action if specified
                    if (field.actions?.onValidate) {
                        try {
                            await executeAction(
                                field.actions.onValidate,
                                {
                                    field: field.key,
                                    value: formData[field.key],
                                    formData,
                                },
                                fieldContext,
                                `field-${field.key}-validate`,
                            );
                        } catch (error) {
                            newErrors[field.key] = (error as Error).message;
                            isValid = false;
                        }
                    }
                }
            }
        }

        errors = newErrors;
        return isValid;
    };

    // Handle form submission
    const handleSubmit = async (event: Event) => {
        event.preventDefault();

        const context = createContext();

        // Check form-level submission policies
        const canSubmit = registry.checkPolicies(schema.policies, context);
        if (!canSubmit.allowed) {
            alert(`Cannot submit: ${canSubmit.reason}`);
            return;
        }

        // Execute submit action
        if (schema.actions?.onSubmit) {
            await executeAction(
                schema.actions.onSubmit,
                formData,
                context,
                "form-submit",
            );
        } else {
            // Default submission behavior
            alert(JSON.stringify(formData, null, 2));
        }
    };

    // Resolve field default value
    const resolveDefault = (field: any, context: PolicyContext) => {
        if (typeof field.default === "function") {
            return field.default(context);
        }
        return field.default;
    };

    // Resolve field props
    const resolveProps = (field: any, context: PolicyContext) => {
        if (typeof field.props === "function") {
            return field.props(context);
        }
        return field.props || {};
    };

    // Initialize form
    onMount(async () => {
        // Set default values for fields that don't have values
        for (const column of schema.columns) {
            for (const group of column) {
                for (const field of group.fields) {
                    if (formData[field.key] === undefined) {
                        const context = createContext(field, group);
                        const defaultValue = resolveDefault(field, context);
                        if (defaultValue !== undefined) {
                            formData[field.key] = defaultValue;
                        }
                    }
                }
            }
        }
        formData = { ...formData };

        // Execute mount action
        if (schema.actions?.onMount) {
            const context = createContext();
            await executeAction(
                schema.actions.onMount,
                formData,
                context,
                "form-mount",
            );
        }
    });

    // Reactive statement for form data changes
    $: {
        // This will run whenever formData changes
        console.log("Form Data Updated:", formData);
    }
</script>

<form on:submit={handleSubmit} class:loading>
    {#if schema.title}<h2>{schema.title}</h2>{/if}
    {#if schema.description}<p class="description">{schema.description}</p>{/if}

    <div class="form-actions">
        <button type="submit" disabled={loading}>
            {loading ? "Processing..." : "Submit"}
        </button>
    </div>

    <div class="form-body">
        {#each schema.columns as column}
            <div class="column">
                {#each column as group}
                    {@const groupContext = createContext(null, group)}
                    {@const groupPolicy = registry.checkPolicies(
                        group.policies,
                        groupContext,
                    )}

                    {#if groupPolicy.allowed}
                        <fieldset class="group">
                            {#if group.title}<legend>{group.title}</legend>{/if}
                            {#if group.description}<p class="group-description">
                                    {group.description}
                                </p>{/if}

                            {#each group.fields as field}
                                {@const fieldModule = registry.get(
                                    field.module || field.type || "text-input",
                                )}
                                {@const fieldContext = createContext(
                                    field,
                                    group,
                                )}
                                {@const fieldPolicy = registry.checkPolicies(
                                    field.policies,
                                    fieldContext,
                                )}

                                {#if fieldModule && fieldPolicy.allowed}
                                    {@const fieldProps = resolveProps(
                                        field,
                                        fieldContext,
                                    )}

                                    <div class="field-wrapper">
                                        <svelte:component
                                            this={fieldModule.component}
                                            label={field.title}
                                            placeholder={field.placeholder}
                                            description={field.description}
                                            value={formData[field.key]}
                                            error={errors[field.key]}
                                            on:input={(e: {
                                                detail: any;
                                                target: { value: any };
                                            }) =>
                                                handleFieldUpdate(
                                                    field,
                                                    e.detail || e.target.value,
                                                )}
                                            {...fieldModule.defaultProps}
                                            {...fieldProps}
                                        />

                                        {#if errors[field.key]}
                                            <span class="error"
                                                >{errors[field.key]}</span
                                            >
                                        {/if}
                                    </div>
                                {/if}
                            {/each}
                        </fieldset>
                    {/if}
                {/each}
            </div>
        {/each}
    </div>
</form>

<style>
    form {
        max-width: 800px;
        margin: 0 auto;
    }

    form.loading {
        opacity: 0.7;
        pointer-events: none;
    }

    .description {
        color: #666;
        margin-bottom: 2rem;
    }

    .form-body {
        display: flex;
        gap: 8px;
        flex-direction: row;
    }

    .column {
        margin-bottom: 2rem;
    }

    .group {
        border: 1px solid #e1e5e9;
        border-radius: 8px;
        padding: 1.5rem;
        margin-bottom: 1.5rem;
        background: #fafbfc;
    }

    legend {
        font-weight: 600;
        font-size: 1.1rem;
        padding: 0 0.5rem;
        color: #2c3e50;
    }

    .group-description {
        color: #666;
        font-size: 0.9rem;
        margin-bottom: 1rem;
    }

    .field-wrapper {
        margin-bottom: 1.5rem;
    }

    .error {
        color: #e74c3c;
        font-size: 0.85rem;
        display: block;
        margin-top: 0.25rem;
    }

    .form-actions {
        text-align: center;
        margin-bottom: 2rem;
        padding-bottom: 2rem;
        border-bottom: 1px solid #e1e5e9;
    }

    button[type="submit"] {
        background: #3498db;
        color: white;
        border: none;
        padding: 0.75rem 2rem;
        border-radius: 6px;
        font-size: 1rem;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    button[type="submit"]:hover:not(:disabled) {
        background: #2980b9;
    }

    button[type="submit"]:disabled {
        background: #bdc3c7;
        cursor: not-allowed;
    }
</style>
