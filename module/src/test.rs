pub use crate::{Context, Error, Merge};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Merged(pub bool);

impl Merge for Merged {
    fn merge_ref(&mut self, _: Self) -> Result<(), Error> {
        self.0 = true;
        Ok(())
    }
}
