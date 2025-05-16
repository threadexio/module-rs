//! Types implementing various merge strategies.

pub mod lines;
pub mod no_merge;
pub mod overridable;

#[doc(inline)]
pub use self::lines::Lines;
#[doc(inline)]
pub use self::no_merge::NoMerge;
#[doc(inline)]
pub use self::overridable::Overridable;

#[allow(unused_imports)]
mod prelude {
    pub(super) use crate::merge::{Context, Error, Merge};

    macro_rules! merge_thin_wrapper {
        (
            $(#[$attr:meta])*
            $vis:vis struct $wrapper:ident;
        ) => {
            $(#[$attr])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $vis struct $wrapper<T>(pub T);

            impl<T> ::core::default::Default for $wrapper<T>
            where
                T: ::core::default::Default
            {
                fn default() -> Self {
                    Self(T::default())
                }
            }

            impl<T> ::core::convert::From<T> for $wrapper<T> {
                #[inline]
                fn from(x: T) -> Self {
                    Self(x)
                }
            }

            impl<T> ::core::borrow::Borrow<T> for $wrapper<T> {
                #[inline]
                fn borrow(&self) -> &T {
                    &self.0
                }
            }

            impl<T> ::core::borrow::BorrowMut<T> for $wrapper<T> {
                #[inline]
                fn borrow_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }

            impl<T> ::core::convert::AsRef<T> for $wrapper<T> {
                #[inline]
                fn as_ref(&self) -> &T {
                    &self.0
                }
            }

            impl<T> ::core::convert::AsMut<T> for $wrapper<T> {
                #[inline]
                fn as_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }

            impl<T> ::core::ops::Deref for $wrapper<T> {
                type Target = T;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> ::core::ops::DerefMut for $wrapper<T> {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
    }

    pub(super) use merge_thin_wrapper;
}
