use super::prelude::*;

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            (Some(a), Some(b)) => a.merge(b).map(Some),
            (x, None) | (None, x) => Ok(x),
        }
    }
}
