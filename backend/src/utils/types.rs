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
