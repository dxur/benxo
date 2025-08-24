import { isEqual, cloneDeep } from 'lodash';
import { snakeToTitleCase } from './fmt';
import * as yup from 'yup';

// Form field values are always concrete, never undefined/null
type FormField<T> = {
    value: T extends never ? string : T;
    initialValue: NonNullable<T> extends never ? string : NonNullable<T>;
    valid: boolean | undefined;
    errors: string[] | undefined;
    validate(): T | undefined;
    hasChanged(): boolean;
};

export type Form<T extends yup.ObjectSchema<any>> = T extends yup.ObjectSchema<infer Shape>
    ? { [K in keyof Shape]: FormField<Shape[K]> }
    : never;

function getConcreteDefault<T>(schema: yup.Schema, providedValue?: T): NonNullable<T> extends never ? string : NonNullable<T> {
    // If provided value is not null/undefined, use it
    if (providedValue !== null && providedValue !== undefined) {
        return providedValue as any;
    }

    // Try to get default from schema
    if (schema.spec?.default !== undefined) {
        const defaultValue = typeof schema.spec.default === 'function'
            ? schema.spec.default()
            : schema.spec.default;

        if (defaultValue !== null && defaultValue !== undefined) {
            return defaultValue as any;
        }
    }

    // Fallback to type-appropriate non-null defaults based on schema type
    const schemaType = schema.describe().type;
    switch (schemaType) {
        case 'string': return '' as any;
        case 'number': return 0 as any;
        case 'boolean': return false as any;
        case 'array': return [] as any;
        case 'object': return {} as any;
        case 'date': return new Date() as any;
        default: return '' as any; // Default to empty string for unknown types
    }
}

function createFormField<T>(schema: yup.Schema, initialValue?: T): FormField<T> {
    const concreteValue = getConcreteDefault(schema, initialValue);

    return {
        value: cloneDeep(concreteValue),
        initialValue: cloneDeep(concreteValue),
        valid: undefined as boolean | undefined,
        errors: undefined as string[] | undefined,
        validate() {
            try {
                // For validation, we pass the actual value (which might need to be null for validation)
                // but the form field value itself is never null/undefined
                let valueToValidate = this.value;

                // Handle special cases where empty values should be treated as null for validation
                if (valueToValidate === '' && schema.describe().nullable) {
                    valueToValidate = null as any;
                }

                const validatedValue = schema.validateSync(valueToValidate, { abortEarly: false });
                this.errors = undefined;
                this.valid = true;
                return validatedValue;
            } catch (e) {
                if (e instanceof yup.ValidationError) {
                    this.errors = e.errors || [e.message];
                    this.valid = false;
                }
                return undefined;
            }
        },
        hasChanged() {
            return !isEqual(this.value, this.initialValue);
        }
    };
}

export function createForm<T extends yup.ObjectSchema<any>, D>(
    schema: T,
    initial?: D & Partial<yup.InferType<T>>
): {
    form: Form<T>;
    data: D | undefined;
} {
    const shape = schema.fields;
    const form: any = {};

    Object.entries(shape).forEach(([key, fieldSchema]) => {
        const initialFieldValue = initial && typeof initial === 'object' && key in initial
            ? (initial as any)[key]
            : undefined;
        form[key] = createFormField(fieldSchema as yup.Schema, initialFieldValue);
    });

    return { form, data: initial };
}

export function validate<T>(field: FormField<T>) {
    return () => {
        field.validate();
    };
}

export function validateForm<T extends yup.ObjectSchema<any>>(form: Form<T>): boolean {
    let allValid = true;

    Object.values(form).forEach((field: FormField<any>) => {
        field.validate();
        if (field.valid === false) {
            allValid = false;
        }
    });

    return allValid;
}

export function assertForm<T extends yup.ObjectSchema<any>>(form: Form<T>) {
    const errors: [string, string[]][] = [];

    Object.entries(form).forEach(([key, field]: [string, FormField<any>]) => {
        field.validate();
        if (field.valid === false && field.errors) {
            errors.push([snakeToTitleCase(key), field.errors]);
        }
    });

    if (errors.length) {
        throw errors;
    }
}

export function getFormValues<T extends yup.ObjectSchema<any>>(
    form: Form<T>,
): yup.InferType<T> {
    const errors: [string, string[]][] = [];
    const values: any = {};

    Object.entries(form).forEach(([key, field]: [string, FormField<any>]) => {
        const validatedValue = field.validate();
        if (field.valid === false && field.errors) {
            errors.push([key, field.errors]);
        }
        if (validatedValue !== undefined) {
            values[key] = validatedValue;
        }
    });

    if (errors.length) {
        throw errors;
    }
    return values;
}

export function getChangedValues<T extends yup.ObjectSchema<any>>(
    form: Form<T>,
): Partial<yup.InferType<T>> | null {
    const errors: [string, string[]][] = [];
    const changedValues: any = {};

    Object.entries(form).forEach(([key, field]: [string, FormField<any>]) => {
        const validatedValue = field.validate();
        if (field.valid === false && field.errors) {
            errors.push([key, field.errors]);
        }

        if (field.hasChanged() && validatedValue !== undefined) {
            changedValues[key] = validatedValue;
        }
    });

    if (errors.length) {
        throw errors;
    }

    return Object.keys(changedValues).length > 0 ? changedValues : null;
}

export function hasFormChanges<T extends yup.ObjectSchema<any>>(form: Form<T>): boolean {
    return Object.values(form).some((field: FormField<any>) => field.hasChanged());
}

export function resetForm<T extends yup.ObjectSchema<any>>(form: Form<T>): void {
    Object.values(form).forEach((field: FormField<any>) => {
        field.value = cloneDeep(field.initialValue);
        field.valid = undefined;
        field.errors = undefined;
    });
}

export function isFormValid<T extends yup.ObjectSchema<any>>(form: Form<T>): boolean {
    return Object.values(form).every((field: FormField<any>) => {
        return field.valid === undefined || field.valid === true;
    });
}

export function getFormErrors<T extends yup.ObjectSchema<any>>(form: Form<T>): Record<string, string[]> {
    const errors: Record<string, string[]> = {};

    Object.entries(form).forEach(([key, field]: [string, FormField<any>]) => {
        if (field.errors && field.errors.length > 0) {
            errors[key] = field.errors;
        }
    });

    return errors;
}