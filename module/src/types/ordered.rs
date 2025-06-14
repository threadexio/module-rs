//! Ordered merging of values.
//!
//! See: [`Ordered`]

use core::mem::swap;

use super::prelude::*;

/// The order of an [`Ordered`] value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Order {
    /// This value should come before the other value.
    Before,

    /// This value should come after the other value.
    After,
}

impl Default for Order {
    #[inline]
    fn default() -> Self {
        Self::Before
    }
}

/// A value which preserves ordering.
///
/// This is a wrapper that provides a way to control the order by which values
/// are merged. Generally, this type implements the following logic:
///
/// * if `order` is [`Before`], then merge this value with the other value,
/// * if `order` is [`After`], then merge the other value with this value
///
/// While at first this might seem pointless, consider the use case where the
/// [`Merge`] implementation of `T` takes into account which value is `self` and
/// which one is `other`. For example, lists. This type can control if the items
/// are added in the front or the back of the list, "before" or "after" the
/// already-existing elements.
///
/// # Example
///
/// ```rust
/// # use module::types::ordered::{Ordered, Order};
/// # use module::merge::Merge;
/// let a = Ordered::with_order(vec![0, 1, 2], Order::After);
/// let b = Ordered::with_order(vec![3, 4, 5], Order::Before);
///
/// let merged = a.merge(b).unwrap();
///
/// assert_eq!(*merged, &[3, 4, 5, 0, 1, 2]);
/// ```
///
/// # serde
///
/// This type deserializes as one of the following:
///
/// * `T`
/// * `{ value: T }`
/// * `{ value: T, order: "before"|"after" }`
///
/// [`Before`]: Order::Before
/// [`After`]: Order::After
#[derive(Debug, Default, Clone, Copy)]
pub struct Ordered<T> {
    value: T,
    order: Order,
}

impl<T> Ordered<T> {
    /// Create a new `value` with the default order.
    #[inline]
    pub fn new(value: T) -> Self {
        Self::with_order(value, Order::default())
    }

    /// Create a new `value` with `order`.
    #[inline]
    pub fn with_order(value: T, order: Order) -> Self {
        Self { value, order }
    }

    /// Get the order of this value.
    #[inline]
    pub fn order(&self) -> Order {
        self.order
    }

    /// Set the order of this value.
    #[inline]
    pub fn set_order<O>(&mut self, order: O)
    where
        O: Into<Order>,
    {
        self.order = order.into();
    }

    /// Destruct this [`Ordered`] and get the inner value.
    #[inline]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> Merge for Ordered<T>
where
    T: Merge,
{
    fn merge_ref(&mut self, mut other: Self) -> Result<(), Error> {
        if other.order == Order::Before {
            swap(&mut self.value, &mut other.value);
        }

        self.value.merge_ref(other.value)
    }
}

impl<T> From<T> for Ordered<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl_wrapper!(Ordered<T> => T { .value });

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::Deserialize;
    use serde::de::Deserializer;

    #[derive(Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum OrderRepr {
        Before,
        After,
    }

    impl From<OrderRepr> for Order {
        fn from(x: OrderRepr) -> Self {
            match x {
                OrderRepr::Before => Order::Before,
                OrderRepr::After => Order::After,
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Repr<T> {
        Order { value: T, order: OrderRepr },
        Value { value: T },
        Raw(T),
    }

    impl<T> From<Repr<T>> for Ordered<T> {
        fn from(x: Repr<T>) -> Self {
            match x {
                Repr::Order { value, order } => Self::with_order(value, Order::from(order)),
                Repr::Value { value } => Ordered::new(value),
                Repr::Raw(value) => Ordered::new(value),
            }
        }
    }

    impl<'de, T> Deserialize<'de> for Ordered<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <Repr<T> as Deserialize>::deserialize(deserializer).map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Order::{After, Before};

    #[inline]
    fn x<T>(value: T, order: Order) -> Ordered<T> {
        Ordered::with_order(value, order)
    }

    #[test]
    fn test_before_after() {
        let a = x(vec![0, 1, 2, 3], Before);
        let b = x(vec![4, 5, 6], After);

        let c = a.merge(b).unwrap();
        assert_eq!(*c, &[0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_after_before() {
        let a = x(vec![0, 1, 2, 3], After);
        let b = x(vec![4, 5, 6], Before);

        let c = a.merge(b).unwrap();
        assert_eq!(*c, &[4, 5, 6, 0, 1, 2, 3]);
    }

    #[test]
    fn test_after_after() {
        let a = x(vec![0, 1, 2, 3], Before);
        let b = x(vec![4, 5, 6], After);

        let c = a.merge(b).unwrap();
        assert_eq!(*c, &[0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_before_before() {
        let a = x(vec![0, 1, 2, 3], Before);
        let b = x(vec![4, 5, 6], Before);

        let c = a.merge(b).unwrap();
        assert_eq!(*c, &[4, 5, 6, 0, 1, 2, 3]);
    }
}
