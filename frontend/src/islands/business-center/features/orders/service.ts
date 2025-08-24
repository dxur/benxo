import * as yup from 'yup';
import { createMutation, createQuery } from "@tanstack/svelte-query";
import type { OrderDto } from "@bindings/OrderDto";
import type { OrderListQuery } from "@bindings/OrderListQuery";
import type { OrderCreate } from "@bindings/OrderCreate";
import type { OrderUpdate } from "@bindings/OrderUpdate";
import type { OrderStatusUpdate } from "@bindings/OrderStatusUpdate";
import type { OrderAnalytics } from "@bindings/OrderAnalytics";
import type { BulkOrderStatusUpdate } from "@bindings/BulkOrderStatusUpdate";
import type { AnalyticsQuery } from "@bindings/AnalyticsQuery";
import { nullIf } from '../../lib/utils/fmt';

import {
    create_order as createOrder,
    get_order as fetchOrder,
    list_orders as listOrders,
    update_order as updateOrder,
    update_order_status as updateOrderStatus,
    bulk_update_order_status as bulkUpdateOrderStatus,
    get_analytics as getAnalytics,
} from '@bindings/OrderRoutes';
import type { OrderListResponse } from '@bindings/OrderListResponse';
import type { BulkUpdateResponse } from '@bindings/BulkUpdateResponse';

const nullStr = nullIf("");

// Shipping Address Schema
const ShippingAddressSchema = yup.object({
    full_name: yup
        .string()
        .required("Full name is required.")
        .min(1, "Full name cannot be empty.")
        .max(100, "Full name cannot exceed 100 characters."),

    address_line_1: yup
        .string()
        .required("Address line 1 is required.")
        .min(1, "Address line 1 cannot be empty.")
        .max(100, "Address line 1 cannot exceed 100 characters."),

    address_line_2: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .max(100, "Address line 2 cannot exceed 100 characters."),

    city: yup
        .string()
        .required("City is required.")
        .min(1, "City cannot be empty.")
        .max(50, "City cannot exceed 50 characters."),

    state: yup
        .string()
        .required("State is required.")
        .max(50, "State cannot exceed 50 characters."),

    postal_code: yup
        .string()
        .required("Postal code is required.")
        .min(1, "Postal code cannot be empty.")
        .max(20, "Postal code cannot exceed 20 characters."),

    country: yup
        .string()
        .required("Country is required.")
        .min(2, "Country must be at least 2 characters.")
        .max(50, "Country cannot exceed 50 characters."),

    phone: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .matches(/^[\+]?[1-9][\d]{0,15}$/, {
            message: "Please enter a valid phone number.",
            excludeEmptyString: true
        }),
});

// Order Item Schema
const OrderItemCreateSchema = yup.object({
    product_id: yup
        .string()
        .required("Product ID is required."),

    variant_sku: yup
        .string()
        .required("Variant SKU is required.")
        .min(1, "Variant SKU cannot be empty.")
        .max(50, "Variant SKU cannot exceed 50 characters."),

    quantity: yup
        .number()
        .integer("Quantity must be a whole number.")
        .min(1, "Quantity must be at least 1.")
        .max(1000, "Quantity cannot exceed 1000.")
        .required("Quantity is required."),
});

// Main Order Create Schema
export const OrderCreateSchema = yup.object({
    customer_email: yup
        .string()
        .email("Please enter a valid email address.")
        .nullable()
        .defined()
        .max(100, "Email cannot exceed 100 characters."),

    customer_name: yup
        .string()
        .required("Customer name is required.")
        .min(1, "Customer name cannot be empty.")
        .max(100, "Customer name cannot exceed 100 characters."),

    customer_phone: yup
        .string()
        .required("Customer phone is required")
        .matches(/^[\+]?[1-9][\d]{0,15}$/, {
            message: "Please enter a valid phone number.",
            excludeEmptyString: true
        }),

    items: yup
        .array(OrderItemCreateSchema)
        .min(1, "Order must have at least one item.")
        .max(50, "Order cannot have more than 50 items.")
        .required("Order items are required."),

    shipping_address: ShippingAddressSchema.required("Shipping address is required."),

    billing_address: ShippingAddressSchema
        .nullable()
        .defined(),

    shipping_cost: yup
        .string()
        .required("Shipping cost is required.")
        .matches(/^\d+(\.\d{1,2})?$/, "Shipping cost must be a valid decimal (e.g., 5.99)."),

    tax_amount: yup
        .string()
        .required("Tax amount is required.")
        .matches(/^\d+(\.\d{1,2})?$/, "Tax amount must be a valid decimal (e.g., 1.50)."),

    currency: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .matches(/^[A-Z]{3}$/, {
            message: "Currency must be a 3-letter code (e.g., USD, EUR).",
            excludeEmptyString: true
        })
        .default("USD"),

    notes: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .max(500, "Notes cannot exceed 500 characters."),
});

// Order Status Update Schema
export const OrderStatusUpdateSchema = yup.object({
    status: yup
        .string<"pending" | "confirmed" | "processing" | "shipped" | "delivered" | "cancelled" | "refunded" | "archived">()
        .oneOf(["pending", "confirmed", "processing", "shipped", "delivered", "cancelled", "refunded", "archived"])
        .required("Status is required."),

    note: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .max(500, "Note cannot exceed 500 characters."),

    created_by: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr),
});

// Order Update Schema
export const OrderUpdateSchema = yup.object({
    status: yup
        .string<"pending" | "confirmed" | "processing" | "shipped" | "delivered" | "cancelled" | "refunded" | "archived">()
        .oneOf(["pending", "confirmed", "processing", "shipped", "delivered", "cancelled", "refunded", "archived"])
        .nullable()
        .defined(),

    // payment_status: yup
    //     .string<"pending" | "paid" | "failed" | "refunded" | "partially_refunded">()
    //     .oneOf(["pending", "paid", "failed", "refunded", "partially_refunded"])
    //     .nullable()
    //     .defined(),

    tracking_number: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .max(100, "Tracking number cannot exceed 100 characters."),

    notes: yup
        .string()
        .nullable()
        .defined()
        .transform(nullStr)
        .max(500, "Notes cannot exceed 500 characters."),

    shipping_address: ShippingAddressSchema
        .nullable()
        .defined(),

    billing_address: ShippingAddressSchema
        .nullable()
        .defined(),
});

// Helper functions
export function getStatusBadgeVariant(status: string) {
    switch (status) {
        case "pending":
        case "confirmed":
            return "outline";
        case "processing":
        case "shipped":
            return "secondary";
        case "delivered":
            return "default";
        case "cancelled":
        case "refunded":
        case "archived":
            return "destructive";
        default:
            return "outline";
    }
}

export function getPaymentStatusBadgeVariant(status: string) {
    switch (status) {
        case "paid":
            return "default";
        case "pending":
            return "outline";
        case "failed":
        case "refunded":
        case "partially_refunded":
            return "destructive";
        default:
            return "outline";
    }
}

export function canCancelOrder(order: OrderDto): boolean {
    return ["pending", "confirmed"].includes(order.status);
}

// export function canRefundOrder(order: OrderDto): boolean {
//     return order.payment_status === "paid" && ["delivered", "shipped"].includes(order.status);
// }

export function canArchiveOrder(order: OrderDto): boolean {
    return ["cancelled", "refunded", "delivered"].includes(order.status);
}

export function formatOrderNumber(id: string): string {
    // Extract last 8 characters and format as order number
    const short = id.slice(-8).toUpperCase();
    return `#${short.slice(0, 4)}-${short.slice(4)}`;
}

export function createDefaultShippingAddress() {
    return {
        full_name: "",
        address_line_1: "",
        address_line_2: null,
        city: "",
        state: "",
        postal_code: "",
        country: "",
        phone: "",
    };
}

export function createDefaultOrderItem() {
    return {
        product_id: "",
        variant_sku: "",
        quantity: 1,
    };
}

// Query hooks
export function useOrderListQuery(getParams: () => OrderListQuery) {
    return createQuery(() => ({
        queryKey: ["orders", getParams()],
        queryFn: () => listOrders(getParams()),
        placeholderData: (prev) => prev,
    }));
}

export function useOrderQuery(orderId: string) {
    return createQuery(() => ({
        queryKey: ["order", orderId],
        queryFn: () => fetchOrder(orderId),
        enabled: !!orderId,
    }));
}

export function useOrderAnalyticsQuery(getParams: () => AnalyticsQuery) {
    const params = getParams();
    return createQuery(() => ({
        queryKey: ["order-analytics", params],
        queryFn: () => getAnalytics(params),
    }));
}

// Mutation hooks
export function useOrderCreate(
    onSuccess: (data: OrderDto) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationFn: (values: OrderCreate) => createOrder(values),
        onSuccess,
        onError,
    }));
}

export function useOrderUpdate(
    orderId: string,
    onSuccess: (data: OrderDto) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationKey: ["order-update", orderId],
        mutationFn: (updateReq: OrderUpdate) => updateOrder(orderId, updateReq),
        onSuccess,
        onError,
    }));
}

export function useOrderStatusUpdate(
    orderId: string,
    onSuccess: (data: OrderDto) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationKey: ["order-status-update", orderId],
        mutationFn: (statusUpdate: OrderStatusUpdate) => updateOrderStatus(orderId, statusUpdate),
        onSuccess,
        onError,
    }));
}

export function useBulkOrderStatusUpdate(
    onSuccess: (data: BulkUpdateResponse) => void,
    onError: (error: Error) => void
) {
    return createMutation(() => ({
        mutationFn: (bulkUpdate: BulkOrderStatusUpdate) => bulkUpdateOrderStatus(bulkUpdate),
        onSuccess,
        onError,
    }));
}
