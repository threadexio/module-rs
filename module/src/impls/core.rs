use super::prelude::*;

unmergeable! {
    bool, char,
    f32, f64,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,

    &[u8], &str,
    core::time::Duration,
    core::net::IpAddr, core::net::Ipv4Addr, core::net::Ipv6Addr,
    core::num::NonZeroI8, core::num::NonZeroI16, core::num::NonZeroI32, core::num::NonZeroI64, core::num::NonZeroIsize,
    core::num::NonZeroU8, core::num::NonZeroU16, core::num::NonZeroU32, core::num::NonZeroU64, core::num::NonZeroUsize,
    core::num::Saturating<i8>, core::num::Saturating<i16>, core::num::Saturating<i32>, core::num::Saturating<i64>, core::num::Saturating<isize>,
    core::num::Saturating<u8>, core::num::Saturating<u16>, core::num::Saturating<u32>, core::num::Saturating<u64>, core::num::Saturating<usize>,
    core::net::SocketAddr, core::net::SocketAddrV4, core::net::SocketAddrV6,
    core::num::Wrapping<i8>, core::num::Wrapping<i16>, core::num::Wrapping<i32>, core::num::Wrapping<i64>, core::num::Wrapping<isize>,
    core::num::Wrapping<u8>, core::num::Wrapping<u16>, core::num::Wrapping<u32>, core::num::Wrapping<u64>, core::num::Wrapping<usize>
}

impl Merge for () {
    fn merge(self, (): Self) -> Result<Self, Error> {
        Ok(())
    }
}

impl<T> Merge for core::cell::Cell<T>
where
    T: Merge,
{
    fn merge(self, other: Self) -> Result<Self, Error> {
        self.into_inner().merge(other.into_inner()).map(Self::new)
    }
}

impl<T> Merge for core::marker::PhantomData<T> {
    fn merge(self, _: Self) -> Result<Self, Error> {
        Ok(Self)
    }
}

impl<T> Merge for core::ops::Range<T> {
    unmergeable!();
}

impl<T> Merge for core::ops::RangeFrom<T> {
    unmergeable!();
}

impl<T> Merge for core::ops::RangeInclusive<T> {
    unmergeable!();
}

impl<T> Merge for core::ops::RangeTo<T> {
    unmergeable!();
}

impl<T, E> Merge for core::result::Result<T, E> {
    unmergeable!();
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        assert_eq!(Some(42).merge(Some(32)).unwrap_err(), Error::collision());
        assert_eq!(None.merge(Some(42)).unwrap(), Some(42));
        assert_eq!(Some(42).merge(None).unwrap(), Some(42));
        assert_eq!(Option::<i32>::None.merge(None).unwrap(), None);
    }
}
