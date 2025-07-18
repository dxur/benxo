use axum::http::StatusCode;
use serde::Serialize;

use crate::extractors::json::Json;

#[macro_export]
macro_rules! register_model {
    ($model:ty) => {
        impl $crate::db::__ModelRegisteredByMacro for $model {}
        #[linkme::distributed_slice($crate::db::MODELS_INIT)]
        pub static __INIT_MODEL: $crate::db::ModelInitFn = |db| Box::pin(<$model>::init_coll(db));
    };
}

pub trait ContainsJson<T> {}

impl<T: Serialize> ContainsJson<T> for Json<T> {}
impl<T: Serialize, E> ContainsJson<T> for Result<Json<T>, E> {}
impl<T: Serialize> ContainsJson<T> for (StatusCode, Json<T>) {}
impl<T: Serialize, E> ContainsJson<T> for Result<(StatusCode, Json<T>), E> {}
