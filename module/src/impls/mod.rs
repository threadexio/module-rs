mod alloc;
mod core;

#[cfg(feature = "std")]
mod std;

mod prelude {
    pub(super) use crate::error::Error;
    pub(super) use crate::merge::Merge;

    macro_rules! unmergeable {
        () => {
            fn merge(self, _other: Self) -> Result<Self, Error> {
                Err(Error::collision())
            }

            fn merge_ref(&mut self, _other: Self) -> Result<(), Error> {
                Err(Error::collision())
            }
        };

        ($($t:ty),*) => {
            $(
                impl Merge for $t {
                    unmergeable!();
                }
            )*
        }
    }
    pub(super) use unmergeable;
}
