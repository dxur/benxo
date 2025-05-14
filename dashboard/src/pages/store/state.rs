use leptos::prelude::*;

use crate::utils::LoadingStatus;

#[derive(Clone, Copy)]
pub struct IndexState {
    pub status: RwSignal<LoadingStatus>,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            status: Default::default(),
        };

        state.status.set(LoadingStatus::Ok);
        state
    }
}
