//! Types implementing various merge strategies.

pub mod first;
pub mod last;
pub mod lines;
pub mod no_merge;
pub mod ordered;
pub mod overridable;

#[doc(inline)]
pub use self::first::First;
#[doc(inline)]
pub use self::last::Last;
#[doc(inline)]
pub use self::lines::Lines;
#[doc(inline)]
pub use self::no_merge::NoMerge;
#[doc(inline)]
pub use self::ordered::Ordered;
#[doc(inline)]
pub use self::overridable::Overridable;

#[allow(unused_imports)]
mod prelude {
    pub(super) use crate::merge::{Context, Error, Merge};

    macro_rules! impl_borrow {
        ($t:ident $(<$tp:ident>)? => $u:ty { $($tail:tt)* }) => {
            impl $(<$tp>)? ::core::borrow::Borrow<$u> for $t $(<$tp>)? {
                #[inline]
                fn borrow(&self) -> &$u {
                    &self $($tail)*
                }
            }

            impl $(<$tp>)? ::core::borrow::BorrowMut<$u> for $t $(<$tp>)? {
                #[inline]
                fn borrow_mut(&mut self) -> &mut $u {
                    &mut self $($tail)*
                }
            }
        }
    }

    pub(super) use impl_borrow;

    macro_rules! impl_as_ref {
        ($t:ident $(<$tp:ident>)? => $u:ty { $($tail:tt)* }) => {
            impl $(<$tp>)? ::core::convert::AsRef<$u> for $t $(<$tp>)? {
                #[inline]
                fn as_ref(&self) -> &$u {
                    &self $($tail)*
                }
            }

            impl $(<$tp>)? ::core::convert::AsMut<$u> for $t $(<$tp>)? {
                #[inline]
                fn as_mut(&mut self) -> &mut $u {
                    &mut self $($tail)*
                }
            }
        }
    }

    pub(super) use impl_as_ref;

    macro_rules! impl_deref {
        ($t:ident $(<$tp:ident>)? => $u:ty { $($tail:tt)* }) => {
            impl $(<$tp>)? ::core::ops::Deref for $t $(<$tp>)? {
                type Target = $u;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self $($tail)*
                }
            }

            impl $(<$tp>)? ::core::ops::DerefMut for $t $(<$tp>)? {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self $($tail)*
                }
            }
        }
    }

    pub(super) use impl_deref;

    macro_rules! impl_wrapper {
        ($($tail:tt)*) => {
            impl_borrow! { $($tail)* }
            impl_as_ref! { $($tail)* }
            impl_deref!  { $($tail)* }
        }
    }

    pub(super) use impl_wrapper;

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

            impl_wrapper!($wrapper<T> => T { .0 });
        };
    }

    pub(super) use merge_thin_wrapper;
}
