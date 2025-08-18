import * as yup from 'yup';
import type { StoreDto } from "@bindings/StoreDto"
import { nullIf } from '../../lib/utils/fmt';

export {
    get_store as fetchStore,
    create_store as createStore,
    list_stores as listStores,
    update_store as updateStore,
    delete_store as deleteStore
} from '@bindings/StoreRoutes';

const nullStr = nullIf("");

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
                    .url("Social link must be a valid URL.")
                    .required("URL is required."),
            })
        )
        .max(10, "You can only add up to 10 social links.")
        .default([]),

    selected_theme: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),

    color_scheme: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),

    header_style: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),

    google_analytics_id: yup
        .string()
        .matches(/^UA-\d{4,9}-\d+$/, { message: "Google Analytics ID must be in the format UA-XXXX-Y.", excludeEmptyString: true })
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

export function canBeDeleted(data?: StoreDto): boolean {
    if (!data) { return false }
    const oneMonthLater = new Date(data?.updated_at);
    oneMonthLater.setMonth(oneMonthLater.getMonth() + 1);
    return new Date() >= oneMonthLater && data.status === "archived";
}
