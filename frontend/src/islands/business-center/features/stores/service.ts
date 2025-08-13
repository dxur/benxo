import { z } from 'zod';
import type { StoreDto } from "@bindings/StoreDto"
import { get_store, create_store, list_stores } from "@bindings/StoreRoutes"
import type { Form } from '../../lib/utils/form';

export {
    get_store as fetchStore,
    create_store as createStore,
    list_stores as listStores
};

export const StoreSchema = z.object({
    status: z.enum(Object.values({
        active: "active",
        archived: "archived",
        draft: "draft",
        inactive: "inactive",
    } as const satisfies { [K in StoreDto['status']]: K }) as [StoreDto["status"]]),
    name: z.string().min(2).max(100),
    description: z.string().max(500),
});

export function canBeDeleted(form: Form<typeof StoreSchema>): boolean {
    const oneMonthLater = new Date(form.data?.updated_at);
    oneMonthLater.setMonth(oneMonthLater.getMonth() + 1);
    return new Date() >= oneMonthLater && form.status.value === "archived";
}