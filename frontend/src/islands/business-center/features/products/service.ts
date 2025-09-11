import * as yup from 'yup';
import { createMutation, createQuery } from "@tanstack/svelte-query";
import type { ProductDto } from "@bindings/ProductDto"
import type { ProductListQuery } from "@bindings/ProductListQuery"
import { nullIf } from '../../lib/utils/fmt';

import {
    get_product as fetchProduct,
    create_product as createProduct,
    list_products as listProducts,
    edit_product as updateProduct,
    delete_product as deleteProduct
} from '@bindings/ProductRoutes';
import type { ProductUpdate } from '@bindings/ProductUpdate';
import { toast } from 'svelte-sonner';
import type { ProductCreateDto } from '@bindings/ProductCreateDto';

const nullStr = nullIf("");

// Product Variant Schema
const ProductVariantSchema = yup.object({
    sku: yup
        .string()
        .required("SKU is required.")
        .min(3, "SKU must be at least 3 characters long.")
        .max(50, "SKU cannot exceed 50 characters.")
        .matches(/^[A-Za-z0-9\-_]+$/, "SKU can only contain letters, numbers, hyphens, and underscores."),

    price: yup
        .string()
        .required("Price is required.")
        .matches(/^\d+(\.\d{1,2})?$/, "Price must be a valid decimal (e.g., 19.99)."),

    compare_at: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .matches(/^\d+(\.\d{1,2})?$/, {
            message: "Compare at price must be a valid decimal (e.g., 29.99).",
            excludeEmptyString: true
        }),

    stocks: yup
        .number()
        .integer("Stock must be a whole number.")
        .min(0, "Stock cannot be negative.")
        .required("Stock quantity is required."),

    images: yup
        .array(yup.string().matches(
            /^(https?:\/\/|data:image\/[a-zA-Z]+;base64,)/,
            "Must be a valid URL or data:image URI"
        ).defined())
        .max(10, "You can only add up to 10 images per variant.")
        .default([]),

    options: yup
        .object()
        .test(
            "variant-options",
            "Option values cannot be empty.",
            function (value) {
                if (!value) return true;

                for (const [key, val] of Object.entries(value)) {
                    if (typeof val === "string" && val.trim().length === 0) {
                        return this.createError({
                            path: `options.${key}`,
                            message: "Option values cannot be empty.",
                        });
                    }
                }
                return true;
            }
        )
        .default({}),
});

// Main Product Schema
export const ProductSchema = yup.object({
    title: yup
        .string()
        .required("Product title is required.")
        .min(3, "Product title must be at least 3 characters long.")
        .max(200, "Product title cannot exceed 200 characters."),

    description: yup
        .string()
        .max(2000, "Description cannot exceed 2000 characters.")
        .default(""),

    status: yup
        .string<"active" | "inactive" | "archived">()
        .oneOf(["active", "inactive", "archived"], "Product status must be one of: active, inactive, archived.")
        .required("Product status is required."),

    category: yup
        .string()
        .max(100, "Category cannot exceed 100 characters.")
        .defined()
        .required("Category is required."),

    featured: yup
        .boolean()
        .default(false),

    images: yup
        .array(yup.string().matches(
            /^(https?:\/\/|data:image\/[a-zA-Z]+;base64,)/,
            "Must be a valid URL or data:image URI"
        ).defined())
        .max(20, "You can only add up to 20 images.")
        .default([]),

    slug: yup
        .string()
        .matches(/^[a-z0-9\-]+$/, {
            message: "Slug can only contain lowercase letters, numbers, and hyphens.",
            excludeEmptyString: true
        })
        .max(100, "Slug cannot exceed 100 characters.")
        .defined()
        .required("Slug is required."),

    variants: yup
        .array(ProductVariantSchema)
        .min(1, "Product must have at least one variant.")
        .max(100, "Product cannot have more than 100 variants.")
        .test(
            "unique-skus",
            "All variant SKUs must be unique.",
            function (variants) {
                if (!variants || variants.length <= 1) return true;

                const skus = variants.map(v => v.sku.toLowerCase());
                const uniqueSkus = new Set(skus);

                if (skus.length !== uniqueSkus.size) {
                    return this.createError({
                        message: "All variant SKUs must be unique.",
                        path: "variants"
                    });
                }
                return true;
            }
        )
        .default([]),
});

export function canBeDeleted(data?: ProductDto): boolean {
    if (!data) { return false }
    const oneMonthLater = new Date(data?.updated_at);
    oneMonthLater.setMonth(oneMonthLater.getMonth() + 1);
    return new Date() >= oneMonthLater && data.status === "archived";
}

export function generateSlug(title: string): string {
    return title
        .toLowerCase()
        .replace(/[^a-z0-9\s\-]/g, '')
        .replace(/\s+/g, '-')
        .replace(/-+/g, '-')
        .trim()
        .substring(0, 100);
}

export function validateVariantCombinations(options: Record<string, string[]>, variants: any[]): string | null {
    if (!options || Object.keys(options).length === 0) return null;

    const optionKeys = Object.keys(options);
    const possibleCombinations = optionKeys.reduce((acc, key) => {
        return acc * options[key].length;
    }, 1);

    if (variants.length > possibleCombinations) {
        return "Too many variants for the available option combinations.";
    }

    // Check for duplicate combinations
    const combinations = new Set();
    for (const variant of variants) {
        const combo = optionKeys.map(key => variant.options[key] || '').join('|');
        if (combinations.has(combo)) {
            return "Duplicate variant combinations detected.";
        }
        combinations.add(combo);
    }

    return null;
}

// Helper function to create default variant
export function createDefaultVariant(): any {
    return {
        sku: '',
        price: '0.00',
        compare_at: null,
        stocks: 0,
        images: [],
        options: {}
    };
}

// Helper function to generate variants from options
export function generateVariantsFromOptions(options: Record<string, string[]>): any[] {
    if (!options || Object.keys(options).length === 0) {
        return [createDefaultVariant()];
    }

    const optionKeys = Object.keys(options);
    const combinations: any[] = [];

    function generateCombinations(index: number, current: Record<string, string>) {
        if (index === optionKeys.length) {
            combinations.push({
                ...createDefaultVariant(),
                options: { ...current }
            });
            return;
        }

        const key = optionKeys[index];
        const values = options[key];

        for (const value of values) {
            generateCombinations(index + 1, { ...current, [key]: value });
        }
    }

    generateCombinations(0, {});
    return combinations;
}

export function useProductListQuery(getParams: () => ProductListQuery) {
    return createQuery(() => {
        const params = getParams();
        return {
            queryKey: ["products", params],
            queryFn: () => listProducts(params),
            placeholderData: (prev) => prev,
        };
    });
}

export function useProductQuery(productId: string) {
    return createQuery(() => ({
        queryKey: ["product", productId],
        queryFn: () => fetchProduct(productId),
    }));
}

export function useProductCreate(
    onSuccess: (data: ProductDto) => void,
    onError: (error: Error) => void
) {

    return createMutation(() => ({
        mutationFn: async (values: ProductCreateDto) => {
            return await createProduct(values);
        },
        onSuccess,
        onError,
    }));
}

export function useProductUpdate(
    productId: string,
    onSuccess: (data: ProductDto) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationKey: ["product", productId],
        mutationFn: (updateReq: ProductUpdate) =>
            updateProduct(productId, updateReq),
        onSuccess,
        onError,
    }))
}

export function useProductDelete(
    productId: string,
    onSuccess: (data: unknown) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationKey: ["product-delete", productId],
        mutationFn: () => deleteProduct(productId),
        onSuccess,
        onError,
    }))
}
