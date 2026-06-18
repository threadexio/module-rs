#[allow(unused_imports)]
pub(crate) use crate::{Context, Error, Merge, merge::ErrorKind};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Merged(pub bool);

impl Merge for Merged {
    fn merge_ref(&mut self, _: Self) -> Result<(), Error> {
        self.0 = true;
        Ok(())
    }
}
