use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

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
    fn into_context(self) -> HaveContext<(), Self>;
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
    fn into_context(self) -> HaveContext<(), Self> {
        self.into()
    }
}

impl<C> From<C> for HaveContext<(), C> {
    fn from(value: C) -> Self {
        Self((), value)
    }
}
