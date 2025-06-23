use axum::Json;
use hyper::StatusCode;
use serde::Serialize;

use crate::models::Void;

use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

impl<T, U> IntoInner<U> for T
where
    T: Into<U>,
{
    fn into_inner(self) -> U {
        self.into()
    }
}

pub trait RefInto<T> {
    fn ref_into(&self) -> T;
}

impl<T, U> RefInto<U> for T
where
    for<'a> &'a T: IntoInner<U>,
{
    fn ref_into(&self) -> U {
        self.into_inner()
    }
}

pub enum AtLeast<F, S> {
    First(F),
    Second(S),
    All(F, S),
}

#[derive(Debug)]
pub struct HaveContext<T, C>(pub T, pub C);

pub trait WithContext<U, C = ()> {
    fn with_context(self, ctx: C) -> HaveContext<U, C>;
}

pub trait IntoContext
where
    Self: Sized,
{
    fn into_context(self) -> HaveContext<Void, Self>;
}

impl<T, U, C> WithContext<U, C> for T
where
    T: Into<U>,
{
    fn with_context(self, ctx: C) -> HaveContext<U, C> {
        HaveContext(self.into(), ctx)
    }
}

impl<T> IntoContext for T {
    fn into_context(self) -> HaveContext<Void, Self> {
        self.into()
    }
}

impl<C> From<C> for HaveContext<Void, C> {
    fn from(value: C) -> Self {
        Self(Void, value)
    }
}

pub trait ResultJsonExt<T> {
    fn into_json(self) -> std::result::Result<Json<T>, StatusCode>;
}

impl<T> ResultJsonExt<T> for std::result::Result<T, StatusCode> {
    fn into_json(self) -> std::result::Result<Json<T>, StatusCode> {
        Ok(Json(self?))
    }
}
