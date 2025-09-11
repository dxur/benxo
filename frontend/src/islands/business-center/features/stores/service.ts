import * as yup from 'yup';
import type { StoreDto } from "@bindings/StoreDto"
import type { StoreRegDto } from "@bindings/StoreRegDto"
import { nullIf, isValidHref } from '../../lib/utils/fmt';

export {
    get_store as fetchStore,
    create_store as createStore,
    list_stores as listStores,
    update_store as updateStore,
    delete_store as deleteStore,
    set_reg as setStoreReg,
    get_reg as getStoreReg,
} from '@bindings/StoreRoutes';

const nullStr = nullIf("");
const defaultStr = (value: unknown) => value ?? "";

export const StoreSchema = yup.object({
    name: yup
        .string()
        .required("Store name is required.")
        .min(2, "Store name must be at least 2 characters long.")
        .max(100, "Store name cannot exceed 100 characters."),

    description: yup
        .string()
        .max(500, "Description cannot exceed 500 characters.")
        .default(""),

    status: yup
        .string<"active" | "inactive" | "archived">()
        .oneOf(["active", "inactive", "archived"], "Store status must be one of: active, inactive, archived.")
        .required("Store status is required."),

    category: yup
        .string()
        .max(100, "Category cannot exceed 100 characters.")
        .defined()
        .nullable()
        .transform(nullStr),

    contact_email: yup
        .string()
        .email("Contact email must be a valid email address.")
        .defined()
        .nullable()
        .transform(nullStr),

    contact_phone: yup
        .string()
        .matches(/^\+?[0-9\s\-]{7,15}$/,
            { message: "Contact phone must be a valid phone number.", excludeEmptyString: true })
        .defined()
        .nullable()
        .transform(nullStr),

    address: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),

    city: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),

    zip_code: yup
        .string()
        .matches(/^[0-9A-Za-z\- ]{3,10}$/,
            { message: "Zip code must be 3-10 characters (letters, numbers, or dashes).", excludeEmptyString: true }
        )
        .nullable()
        .defined()
        .transform(nullStr),

    social_links: yup
        .array(
            yup.object({
                platform: yup
                    .string()
                    .required("Platform name is required."),

                url: yup
                    .string()
                    .test("is-valid-url", "Social link must be a valid URL.", isValidHref)
                    .required("URL is required."),
            })
        )
        .max(10, "You can only add up to 10 social links.")
        .default([]),

    logo: yup
        .string()
        .test("is-valid-url", "Logo URL must be a valid URL.", isValidHref)
        .nullable()
        .defined()
        .transform(nullStr),

    logo_alt: yup
        .string()
        .max(100, "Logo alt text cannot exceed 100 characters.")
        .nullable()
        .defined()
        .transform(nullStr),

    favicon: yup
        .string()
        .test("is-valid-url", "Favicon URL must be a valid URL.", isValidHref)
        .nullable()
        .defined()
        .transform(nullStr),

    menu_items: yup
        .array(
            yup.object({
                label: yup
                    .string()
                    .required("Menu item label is required.")
                    .max(50, "Menu item label cannot exceed 50 characters."),

                url: yup
                    .string()
                    .required("Menu item URL is required.")
                    .test("is-valid-url", "Menu item URL must be a valid URL.", isValidHref),
            })
        )
        .max(20, "You can only add up to 20 menu items.")
        .default([]),

    featured_collections: yup
        .array(
            yup.object({
                label: yup
                    .string()
                    .required("Collection label is required.")
                    .max(100, "Collection label cannot exceed 100 characters."),

                img: yup
                    .string()
                    .test("is-valid-url", "Image must be a valid URL.", isValidHref)
                    .nullable()
                    .defined()
                    .transform(nullStr),
            })
        )
        .max(20, "You can only feature up to 20 collections.")
        .default([]),

    footer_lists: yup
        .array(
            yup.object({
                title: yup
                    .string()
                    .required("Footer list title is required.")
                    .max(50, "Footer list title cannot exceed 50 characters."),

                items: yup
                    .array(
                        yup.object({
                            label: yup
                                .string()
                                .required("Footer item label is required.")
                                .max(50, "Footer item label cannot exceed 50 characters."),

                            url: yup
                                .string()
                                .test("is-valid-url", "Footer item URL must be a valid URL.", isValidHref)
                                .required("Footer item URL is required.")
                        })
                    )
                    .max(20, "You can only add up to 20 items per footer list.")
                    .required("Footer list items are required."),
            })
        )
        .max(5, "You can only add up to 5 footer lists.")
        .default([]),

    homepage_template: yup
        .string()
        .default("")
        .transform(defaultStr),

    product_page_template: yup
        .string()
        .default("")
        .transform(defaultStr),

    cart_page_template: yup
        .string()
        .default("")
        .transform(defaultStr),

    shop_page_template: yup
        .string()
        .default("")
        .transform(defaultStr),

    not_found_page_template: yup
        .string()
        .default("")
        .transform(defaultStr),

    custom_pages: yup
        .object()
        .test("values-are-strings", "All custom pages must be strings.", (obj) =>
            obj ? Object.values(obj).every((v) => typeof v === "string") : true
        )
        .default({}),

    snippets: yup
        .object()
        .test("values-are-strings", "All snippets must be strings.", (obj) =>
            obj ? Object.values(obj).every((v) => typeof v === "string") : true
        )
        .default({}),

    // Replace the existing google_analytics_id validation with:
    google_analytics_id: yup
        .string()
        .matches(/^(UA-\d{4,9}-\d+|G-[A-Z0-9]+)$/, {
            message: "Google Analytics ID must be in UA-XXXX-Y format (Universal Analytics) or G-XXXXXXXX format (GA4).",
            excludeEmptyString: true
        })
        .nullable()
        .defined()
        .transform(nullStr),

    gtm_container_id: yup
        .string()
        .matches(/^GTM-[A-Z0-9]+$/, { message: "GTM Container ID must start with GTM- followed by alphanumeric.", excludeEmptyString: true })
        .nullable()
        .defined()
        .transform(nullStr),

    tracking_pixels: yup
        .array(
            yup.object({
                platform: yup
                    .string()
                    .required("Pixel platform is required."),

                pixel_id: yup
                    .string()
                    .required("Pixel ID is required."),

                events: yup
                    .array(
                        yup
                            .string()
                            .required("Event name cannot be empty.")
                    )
                    .default([]),
            })
        )
        .default([]),

    meta_title: yup
        .string()
        .max(60, "Meta title cannot exceed 60 characters.")
        .nullable()
        .defined()
        .transform(nullStr),

    meta_description: yup
        .string()
        .max(160, "Meta description cannot exceed 160 characters.")
        .nullable()
        .defined()
        .transform(nullStr),

    meta_keywords: yup
        .string()
        .max(255, "Meta keywords cannot exceed 255 characters.")
        .nullable()
        .defined()
        .transform(nullStr),

    custom_key_values: yup
        .object()
        .test(
            "custom-values",
            "Custom values cannot exceed 200 characters.",
            function (value) {
                if (!value) return true;

                for (const [key, val] of Object.entries(value)) {
                    if (typeof val === "string" && val.length > 200) {
                        return this.createError({
                            path: `custom_key_values.${key}`,
                            message: "Custom values cannot exceed 200 characters.",
                        });
                    }
                }
                return true;
            }
        )
        .default({}),
});

// Domain Registration Schema
export const DomainRegSchema = yup.object({
    slug: yup
        .string()
        .required("Store slug is required.")
        .matches(/^[a-z0-9]+(?:-[a-z0-9]+)*$/, "Slug must contain only lowercase letters, numbers, and hyphens (no consecutive hyphens).")
        .min(3, "Slug must be at least 3 characters long.")
        .max(50, "Slug cannot exceed 50 characters."),

    domain: yup
        .string()
        .nullable()
        .matches(
            /^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$/,
            { message: "Please enter a valid domain name.", excludeEmptyString: true }
        )
        .transform(nullStr),
});

export function canBeDeleted(data?: StoreDto): boolean {
    if (!data) { return false }
    const oneMonthLater = new Date(data?.updated_at);
    oneMonthLater.setMonth(oneMonthLater.getMonth() + 1);
    return new Date() >= oneMonthLater && data.status === "archived";
}

export function generateSlugFromName(name: string): string {
    return name
        .toLowerCase()
        .replace(/[^a-z0-9\s-]/g, '') // Remove special characters
        .replace(/\s+/g, '-') // Replace spaces with hyphens
        .replace(/-+/g, '-') // Replace multiple hyphens with single hyphen
        .replace(/^-|-$/g, ''); // Remove leading/trailing hyphens
}