mod alloc;
mod core;

#[cfg(feature = "std")]
mod std;

mod prelude {
    pub(super) use crate::error::Error;
    pub(super) use crate::merge::Merge;
}
