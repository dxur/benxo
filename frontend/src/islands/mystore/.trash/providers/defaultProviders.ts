import type { FieldProvider } from '../lib/schema';

export function registerDefaultProviders(store: any) {
    // Text provider
    store.registerFieldProvider('text', {
        validate: (value, field) => {
            if (field.required && (!value || value.trim() === '')) {
                return `${field.label} is required`;
            }
            return null;
        }
    });

    // Email provider with validation
    store.registerFieldProvider('email', {
        validate: (value, field) => {
            if (field.required && !value) {
                return `${field.label} is required`;
            }
            if (value && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
                return 'Please enter a valid email address';
            }
            return null;
        }
    });

    // File provider with upload
    store.registerFieldProvider('file', {
        updateStrategy: 'immediate',
        onSave: async (value, field, context) => {
            if (context.api?.uploadFile) {
                return await context.api.uploadFile(value, field.key);
            }
            return value;
        }
    });

    // Rich text with debounced save
    store.registerFieldProvider('richtext', {
        updateStrategy: 'debounced',
        debounceMs: 1000,
        transform: (value) => value // Could convert to/from HTML
    });

    // Color with immediate save
    store.registerFieldProvider('color', {
        updateStrategy: 'immediate'
    });

    // Number with validation
    store.registerFieldProvider('number', {
        parse: (value) => value === '' ? null : Number(value),
        validate: (value, field) => {
            if (field.required && (value === null || value === undefined)) {
                return `${field.label} is required`;
            }
            if (field.props?.min !== undefined && value < field.props.min) {
                return `${field.label} must be at least ${field.props.min}`;
            }
            if (field.props?.max !== undefined && value > field.props.max) {
                return `${field.label} must be at most ${field.props.max}`;
            }
            return null;
        }
    });
}
