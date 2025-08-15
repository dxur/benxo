import { camelToTitleCase } from './fmt';
import { z } from 'zod';

function getDefaultValue(schema: z.ZodTypeAny): any {
    if (schema instanceof z.ZodDefault) {
        return schema._def.defaultValue();
    }
    if (schema instanceof z.ZodOptional || schema instanceof z.ZodNullable) {
        return undefined;
    }

    if (schema instanceof z.ZodString) return "";
    if (schema instanceof z.ZodNumber) return 0;
    if (schema instanceof z.ZodBoolean) return false;
    if (schema instanceof z.ZodArray) return [];
    if (schema instanceof z.ZodObject) return {};
    if (schema instanceof z.ZodTuple) return [];
    if (schema instanceof z.ZodRecord) return {};
    if (schema instanceof z.ZodEnum) {
        return schema.options[0];
    }

    return undefined;
}

type FormField<T> = {
    value: T;
    initialValue: T;
    valid: boolean | undefined;
    errors: string[] | undefined;
    validate(): void;
    hasChanged(): boolean;
};

export type Form<T extends z.ZodTypeAny> = T extends z.ZodObject<infer Shape>
    ? { [K in keyof Shape]: Form<Shape[K]> } & { data?: any }
    : FormField<z.infer<T>>;

function deepEqual(a: any, b: any): boolean {
    if (a === b) return true;
    if (a == null || b == null) return false;
    if (typeof a !== typeof b) return false;

    if (Array.isArray(a) && Array.isArray(b)) {
        if (a.length !== b.length) return false;
        return a.every((item, index) => deepEqual(item, b[index]));
    }

    if (typeof a === 'object') {
        const keysA = Object.keys(a);
        const keysB = Object.keys(b);
        if (keysA.length !== keysB.length) return false;
        return keysA.every(key => deepEqual(a[key], b[key]));
    }

    return false;
}

function createInnerForm<T extends z.ZodTypeAny, D extends Partial<z.infer<T>>>(schema: T, initial?: D): Form<T> {
    if (schema instanceof z.ZodObject) {
        const shape = schema.shape;
        return Object.fromEntries(
            Object.entries(shape).map(([key, fieldSchema]) => {
                const initialFieldValue = initial && typeof initial === 'object' && key in initial
                    ? (initial as any)[key]
                    : undefined;
                return [key, createInnerForm(fieldSchema as z.ZodTypeAny, initialFieldValue)];
            })
        ) as Form<T>;
    } else {
        const defaultValue = getDefaultValue(schema);
        const finalValue = initial !== undefined ? initial : defaultValue;

        return {
            value: finalValue,
            initialValue: finalValue,
            valid: undefined as boolean | undefined,
            errors: undefined as string[] | undefined,
            validate() {
                try {
                    schema.parse(this.value);
                    this.errors = undefined;
                    this.valid = true;
                } catch (e) {
                    const err = e as z.ZodError;
                    this.errors = (err.errors || []).map((e: any) => e.message);
                    this.valid = false;
                }
            },
            hasChanged() {
                return !deepEqual(this.value, this.initialValue);
            }
        } as Form<T>;
    }
}

export function createForm<T extends z.ZodTypeAny, D extends Partial<z.infer<T>>>(schema: T, initial?: D): { form: Form<T>; data?: D } {
    return { form: createInnerForm(schema, initial), data: initial }
}


export function validate<T>(field: FormField<T>) {
    return () => {
        field.validate.call(field)
    };
}

export function validateForm<T extends z.ZodObject<any>>(form: Form<T>): boolean {
    let allValid = true;

    function validateRecursive(obj: any): void {
        Object.values(obj).forEach((field: any) => {
            if (typeof field.validate === 'function') {
                field.validate();
                if (field.valid === false) {
                    allValid = false;
                }
            } else if (typeof field === 'object' && field !== null) {
                validateRecursive(field);
            }
        });
    }

    validateRecursive(form);
    return allValid;
}

export function assertForm<T extends z.ZodObject<any>>(form: Form<T>) {
    let errors: [string, string[]][] = [];
    function validateRecursive(obj: any): void {
        Object.entries(obj).forEach(([key, field]: [string, any]) => {
            if (typeof field.validate === 'function') {
                field.validate();
                if (field.valid === false) {
                    errors.push([camelToTitleCase(key), field.errors]);
                }
            } else if (typeof field === 'object' && field !== null) {
                validateRecursive(field);
            }
        });
    }

    validateRecursive(form);
    if (errors.length) {
        throw errors;
    }
}

export function getFormValues<T extends z.ZodObject<any>>(
    form: Form<T>,
): z.infer<T> {
    const errors: [string, string[]][] = [];
    function getValuesRecursive(obj: any): any {
        const values: any = {};
        Object.entries(obj).forEach(([key, field]: [string, any]) => {
            field.validate();
            if (field.valid === false) {
                errors.push([key, field.errors]);
            }
            if (field.value !== undefined) {
                values[key] = field.value;
            } else if (typeof field === 'object' && field !== null) {
                values[key] = getValuesRecursive(field);
            }
        });
        return values;
    }

    const values = getValuesRecursive(form);
    if (errors.length) {
        throw errors;
    }
    return values;

}

export function getChangedValues<T extends z.ZodObject<any>>(
    form: Form<T>,
): Partial<z.infer<T>> | null {
    const errors: [string, string[]][] = [];
    const values: any = {};

    Object.entries(form).forEach(([key, field]: [string, any]) => {
        field.validate();
        if (field.valid === false) {
            errors.push([key, field.errors]);
        }

        if (typeof field?.hasChanged === 'function') {
            if (field.hasChanged()) {
                values[key] = field.value;
            }
        } else if (typeof field === 'object' && field !== null) {
            const nestedChanged = getChangedValues(field);
            if (nestedChanged !== null) {
                values[key] = nestedChanged;
            }
        }
    });

    if (errors.length) {
        throw errors;
    }

    return Object.keys(values).length > 0 ? values : null;
}

export function hasFormChanges<T extends z.ZodObject<any>>(form: Form<T>): boolean {
    function checkChangesRecursive(obj: any): boolean {
        return Object.values(obj).some((field: any) => {
            if (typeof field.hasChanged === 'function') {
                return field.hasChanged();
            } else if (typeof field === 'object' && field !== null) {
                return checkChangesRecursive(field);
            }
            return false;
        });
    }

    return checkChangesRecursive(form);
}

export function resetForm<T extends z.ZodObject<any>>(form: Form<T>): void {
    function resetRecursive(obj: any): void {
        Object.values(obj).forEach((field: any) => {
            if (field.initialValue !== undefined && field.value !== undefined) {
                field.value = field.initialValue;
                field.valid = undefined;
                field.errors = undefined;
            } else if (typeof field === 'object' && field !== null) {
                resetRecursive(field);
            }
        });
    }

    resetRecursive(form);
}

export function isFormValid<T extends z.ZodObject<any>>(form: Form<T>): boolean {
    function checkValidRecursive(obj: any): boolean {
        return Object.values(obj).every((field: any) => {
            if (field.valid !== undefined) {
                return field.valid === true;
            } else if (typeof field === 'object' && field !== null) {
                return checkValidRecursive(field);
            }
            return true;
        });
    }

    return checkValidRecursive(form);
}

export function getFormErrors<T extends z.ZodObject<any>>(form: Form<T>): Record<string, string[]> {
    const errors: Record<string, string[]> = {};

    function collectErrorsRecursive(obj: any, prefix = ''): void {
        Object.entries(obj).forEach(([key, field]: [string, any]) => {
            const fieldKey = prefix ? `${prefix}.${key}` : key;

            if (field.errors && field.errors.length > 0) {
                errors[fieldKey] = field.errors;
            } else if (typeof field === 'object' && field !== null && !field.value && !field.validate) {
                collectErrorsRecursive(field, fieldKey);
            }
        });
    }

    collectErrorsRecursive(form);
    return errors;
}