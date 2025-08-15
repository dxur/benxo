import { z } from 'zod';
import type { StoreDto } from "@bindings/StoreDto"
import type { Form } from '../../lib/utils/form';

export {
    get_store as fetchStore,
    create_store as createStore,
    list_stores as listStores,
    update_store as updateStore
} from '@bindings/StoreRoutes';

export const StoreSchema = z.object({
    status: z.enum(Object.values({
        active: "active",
        inactive: "inactive",
        archived: "archived",
    } as const satisfies { [K in StoreDto['status']]: K }) as [StoreDto["status"]]),
    name: z.string().min(2).max(100),
    description: z.string().max(500),
});

export function canBeDeleted(data?: StoreDto): boolean {
    if (!data) { return false }
    const oneMonthLater = new Date(data?.updated_at);
    oneMonthLater.setMonth(oneMonthLater.getMonth() + 1);
    return new Date() >= oneMonthLater && data.status === "archived";
}
