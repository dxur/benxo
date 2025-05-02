use backend::api::{ApiRoutes, Routes};
use backend::models::product::{ProductCreate, ProductDelete, ProductPublic};
use backend::models::{ObjectId, Page, Pagination};
use leptos::{prelude::*, html::*, task::spawn_local};
use leptos_router::NavigateOptions;

use crate::forms::product::ProductCreateAccessor;
use crate::routes::{AppRoutes, RouteExt};
use crate::utils::LoadingStatus;


#[derive(Clone, Copy)]
pub struct ProductsService {
    pub products: RwSignal<Page<ProductPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub page: RwSignal<usize>,
    pub total: RwSignal<usize>,
    pub create_acc: RwSignal<Option<ProductCreateAccessor>>,
    pub dialog: NodeRef<Dialog>,
}

impl ProductsService {
    pub fn new() -> Self {
        let service = Self {
            products: Default::default(),
            status: Default::default(),
            page: RwSignal::new(1),
            total: Default::default(),
            create_acc: Default::default(),
            dialog: Default::default(),
        };

        Effect::watch(
            move || service.page.get(),
            move |_, _, _| {
                service.fetch_products();
            },
            true,
        );

        Effect::watch(
            move || service.create_acc.get(),
            move |new_acc, _, _| {
                match new_acc {
                    Some(_) => {
                        service.dialog.get_untracked().map(|d| {
                            log::info!("Opening dialog");
                            let _ = d.show_modal();
                        });
                    }
                    None => {
                        service.dialog.get_untracked().map(|d| {
                            log::info!("Closing dialog");
                            d.close();
                        });
                    }
                }
            },
            false,
        );

        service
    }

    pub fn fetch_products(self) {
        self.status.set(LoadingStatus::Loading);
        spawn_local(async move {
            let res = ApiRoutes::get_all_products(
                Pagination {
                    page: Some(self.page.get_untracked()),
                    per_page: None,
                }
            ).await.map_err(|e| e.to_string());
            log::debug!("Fetched products: {:?}", res);
            match res {
                Ok(data) => {
                    self.total.set(data.total_pages());
                    self.products.set(data);
                    self.status.set(LoadingStatus::Ok);
                },
                Err(e) => {
                    self.status.set(LoadingStatus::Err(e));
                },
            }
        });
    }


    pub fn create_product(self, navigate: impl Fn(&str, NavigateOptions) + 'static) {
        let res: Result<ProductCreate, ()> = self.create_acc.get_untracked().unwrap().try_into();
        log::debug!("Into product: {:?}", res);
        match res {
            Ok(product) => spawn_local(async move {
                let res = ApiRoutes::create_product(todo!()).await;
                log::debug!("Created product: {:?}", res);
                match res {
                    Ok(product) => {
                        navigate(
                            AppRoutes::PRODUCT_EDIT.path().replace(":id", &product.id.to_hex()).as_str(),
                            Default::default()
                        );
                    },
                    Err(e) => log::error!("Failed to create product: {}", e),
                }
            }),
            Err(e) => log::error!("Failed to create product: {:?}", e),
        }
    }

    pub fn delete_product(self, id: ObjectId) {
        spawn_local(async move {
            let res = ApiRoutes::delete_product(ProductDelete { id: id }).await;
            log::debug!("Deleted product: {:?}", res);
            match res {
                Ok(_) => {
                    self.fetch_products();
                },
                Err(e) => log::error!("Failed to delete product: {}", e),
            }
        });
    }
}
