pub type Error = String;

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingStatus {
    Loading,
    Ok,
    Err(Error),
}

impl Default for LoadingStatus {
    fn default() -> Self {
        Self::Loading
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn is_subpath(base: &str, current: &str) -> bool {
    let base = base.trim_end_matches('/');
    let current = if current.is_empty() { "/" } else { current };

    if base.is_empty() {
        return current == "/" || current.is_empty();
    }

    if current == base {
        return true;
    }

    if let Some(stripped) = current.strip_prefix(base) {
        return stripped == "/" || stripped.starts_with('/');
    }

    false
}

use std::fmt::Debug;

pub struct DropLogger<T: Debug> {
    name: &'static str,
    value: T,
}

impl<T: Debug> DropLogger<T> {
    pub fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }

    pub fn inner(&self) -> &T {
        &self.value
    }
}

impl<T: Debug> AsRef<T> for DropLogger<T> {
    fn as_ref(&self) -> &T {
        self.inner()
    }
}

impl<T: Debug> std::ops::Deref for DropLogger<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T: Debug> Drop for DropLogger<T> {
    fn drop(&mut self) {
        log::debug!("Dropping `{}` with value: {:?}", self.name, self.value);
    }
}
