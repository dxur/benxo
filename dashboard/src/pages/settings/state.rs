use backend::api::{ApiRoutes, Routes};
use backend::models::order::*;
use backend::models::settings::SettingsPublic;
use backend::models::{ObjectId, Page, Pagination};
use indexmap::IndexMap;
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use std::str::FromStr;

use crate::notifications::{error, success};
use crate::routes::*;
use crate::utils::*;

#[derive(Clone, Copy, Default)]
pub struct Fields {}

#[derive(Clone, Copy)]
pub struct IndexState {
    pub settings: RwSignal<Option<SettingsPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub fields: RwSignal<Fields>,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            settings: Default::default(),
            status: Default::default(),
            fields: Default::default(),
        };

        state.fetch();
        state
    }

    pub fn fetch(self) {
        spawn_local(async move {
            let res = ApiRoutes::get_settings().await;
            log::debug!("Fetched settings: {:?}", res);
            match res {
                Ok(s) => {
                    // self.update_fields(s.clone());
                    self.settings.set(Some(s));
                    self.status.set(LoadingStatus::Ok);
                }
                Err(e) => {
                    self.status.set(LoadingStatus::Err(e.to_string()));
                }
            }
        });
    }
}
