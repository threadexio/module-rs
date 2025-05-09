use core::fmt::Display;

use super::Error;

pub trait Context {
    fn module<D>(self, name: D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized;

    fn with_module<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display + Send + Sync + 'static;

    fn value<D>(self, name: D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized;

    fn with_value<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized;
}

impl<T> Context for core::result::Result<T, Error> {
    fn module<D>(self, name: D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.with_module(|| name)
    }

    fn with_module<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display + Send + Sync + 'static,
    {
        self.map_err(|mut e| {
            e.modules.add(f());
            e
        })
    }

    fn value<D>(self, name: D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.with_value(|| name)
    }

    fn with_value<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.map_err(|mut e| {
            e.value.add(f());
            e
        })
    }
}
