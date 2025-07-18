use axum::Json;
use mongodb::{
    action::{ExplicitSession, Find},
    ClientSession,
};

use super::error::ApiError;
use crate::models::{Page, Void};

pub type Result<T> = std::result::Result<T, ApiError>;

pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

pub trait RefInto<T> {
    fn ref_into(&self) -> T;
}

impl<T, U> RefInto<U> for T
where
    for<'a> &'a T: Into<U>,
{
    fn ref_into(&self) -> U {
        self.into()
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

pub trait ResultJsonExt<T, E> {
    fn into_json(self) -> std::result::Result<Json<T>, E>;
}

impl<T, E> ResultJsonExt<T, E> for std::result::Result<T, E> {
    fn into_json(self) -> std::result::Result<Json<T>, E> {
        Ok(Json(self?))
    }
}

pub trait ResultBodyExt<T, E> {
    fn into_body(self) -> std::result::Result<T, E>;
}

impl<T, U, E> ResultBodyExt<T, E> for std::result::Result<U, E>
where
    U: Into<T>,
{
    fn into_body(self) -> std::result::Result<T, E> {
        self.map(|v| v.into())
    }
}

pub trait ResultPageExt<T, E> {
    fn into_page(self) -> std::result::Result<Page<T>, E>;
}

impl<T, U, E> ResultPageExt<T, E> for std::result::Result<Page<U>, E>
where
    Page<U>: IntoInner<Page<T>>,
{
    fn into_page(self) -> std::result::Result<Page<T>, E> {
        self.map(|v| v.into_inner())
    }
}

pub trait WithSome: Sized {
    fn with_some<T, F: FnOnce(Self, T) -> Self>(self, f: F, arg: Option<T>) -> Self {
        match arg {
            Some(v) => f(self, v),
            None => self,
        }
    }
}

impl<T: Sized> WithSome for T {}

pub trait MongoFindExt<'a, T: Send + Sync, Session> {
    fn some_session(
        self,
        session: Option<&mut ClientSession>,
    ) -> std::result::Result<Find<'a, T, ExplicitSession>, Find<'a, T, Session>>;
}
