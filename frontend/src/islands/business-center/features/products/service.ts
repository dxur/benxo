// product-edit.service.ts
import * as ProductRoutes from "@bindings/ProductRoutes";
import { toast } from "svelte-sonner";
import type { ProductDto } from "@bindings/ProductDto";
import type { ProductUpdate } from "@bindings/ProductUpdate";
import type { ProductVariant } from "@bindings/ProductVariant";
import type { ProductVariantCreate } from "@bindings/ProductVariantCreate";
import { delete_product, edit_product, get_product, list_products } from "@bindings/ProductRoutes";

export class ProductVM {
    id = $state('');
    title = $state('');
    description = $state('');
    status = $state<'draft' | 'active' | 'archived'>('draft');
    featured = $state<boolean>(false);
    category = $state<string>('');
    images = $state<string[]>([]);
    variants = $state<ProductVariantVM[]>([]);
    slug = $state<string>('');

    private original: Partial<ProductVM>;
    isDirty: boolean = false;
    errors: Record<string, string[]> = {};

    constructor(dto?: ProductDto) {
        if (dto) {
            this.id = dto.id as string;
            this.title = dto.title as string;
            this.description = dto.description;
            this.status = dto.status;
            this.featured = dto.featured;
            this.category = dto.category;
            this.images = [...dto.images];
            this.variants = dto.variants.map(v => new ProductVariantVM(v));
            this.slug = dto.slug;
        }
        this.original = { ...this };
    }

    // Simple validation
    validate(): boolean {
        this.errors = {};

        if (!this.title.trim()) {
            this.errors.title = ['Title is required'];
        }

        if (!this.description.trim()) {
            this.errors.description = ['Description is required'];
        }

        return Object.keys(this.errors).length === 0;
    }

    // Convert to update DTO
    toUpdateDto(): ProductUpdate {
        return {
            title: this.title as any,
            description: this.description,
            category: this.category,
            images: [...this.images],
            featured: this.featured,
            status: this.status,
            options: {},
            variants: this.variants.map(v => v.toDto()),
            slug: this.slug
        };
    }

    // Mark as saved
    markSaved(): void {
        this.original = { ...this };
        this.isDirty = false;
    }

    // Check if dirty
    checkDirty(): void {
        this.isDirty = JSON.stringify(this) !== JSON.stringify(this.original);
    }

    // Simple business methods
    addVariant(): void {
        this.variants.push(new ProductVariantVM());
        this.checkDirty();
    }

    removeVariant(index: number): void {
        this.variants.splice(index, 1);
        this.checkDirty();
    }
}

// Simple Product Variant
export class ProductVariantVM {
    sku: string = '';
    price: string = '';
    stocks: number = 0;
    images: string[] = [];

    constructor(variant?: ProductVariant) {
        if (variant) {
            this.sku = variant.sku;
            this.price = variant.price;
            this.stocks = variant.stocks;
            this.images = [...variant.images];
        }
    }

    toDto(): ProductVariantCreate {
        return {
            sku: this.sku,
            price: this.price,
            compare_at: null,
            stocks: this.stocks,
            images: [...this.images],
            options: {}
        };
    }
}

// Page class for Product Edit
export class ProductEditPage {
    product: ProductVM;
    isLoading: boolean = false;
    error: string | null = null;

    constructor(productId?: string) {
        this.product = new ProductVM();
        if (productId) {
            this.loadProduct(productId);
        }
    }

    async loadProduct(id: string): Promise<void> {
        this.isLoading = true;
        this.error = null;
        try {
            const dto = await get_product(id);
            this.product = new ProductVM(dto);
        } catch (err) {
            this.error = `Failed to load product: ${err}`;
        } finally {
            this.isLoading = false;
        }
    }

    async saveProduct(): Promise<void> {
        if (!this.product.validate()) {
            return;
        }

        this.isLoading = true;
        this.error = null;
        try {
            const updateDto = this.product.toUpdateDto();
            await edit_product(this.product.id, updateDto);
            this.product.markSaved();
        } catch (err) {
            this.error = `Failed to save product: ${err}`;
        } finally {
            this.isLoading = false;
        }
    }

    updateTitle(title: string): void {
        this.product.title = title;
        this.product.checkDirty();
    }

    updateDescription(description: string): void {
        this.product.description = description;
        this.product.checkDirty();
    }
}

// Page class for Product List
export class ProductListPage {
    products: ProductVM[] = [];
    isLoading: boolean = false;
    error: string | null = null;

    async loadProducts(): Promise<void> {
        this.isLoading = true;
        this.error = null;
        try {
            const response = await list_products();
            this.products = response.products.map(dto => new ProductVM(dto));
        } catch (err) {
            this.error = `Failed to load products: ${err}`;
        } finally {
            this.isLoading = false;
        }
    }

    async deleteProduct(id: string): Promise<void> {
        this.isLoading = true;
        try {
            await delete_product(id);
            this.products = this.products.filter(p => p.id !== id);
        } catch (err) {
            this.error = `Failed to delete product: ${err}`;
        } finally {
            this.isLoading = false;
        }
    }
}
