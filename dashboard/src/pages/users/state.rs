use backend::api::{ApiRoutes, Routes};
use backend::models::user::*;
use backend::models::{ObjectId, Page, Pagination};
use indexmap::{IndexMap, IndexSet};
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use slotmap::{DefaultKey, SlotMap};
use std::str::FromStr;

use crate::notifications::{error, success};
use crate::routes::*;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct IndexState {
    pub users: RwSignal<Page<UserPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub page: RwSignal<usize>,
    pub total: RwSignal<usize>,
    pub dialog: NodeRef<Dialog>,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            users: Default::default(),
            status: Default::default(),
            page: RwSignal::new(1),
            total: Default::default(),
            dialog: Default::default(),
        };

        Effect::watch(
            move || state.page.get(),
            move |_, _, _| {
                state.fetch();
            },
            true,
        );
        state
    }

    pub fn fetch(self) {
        self.status.set(LoadingStatus::Loading);
    }
}
