#[allow(unused_imports)]
use crate::test::*;

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_unit() {
    #[derive(Merge)]
    struct Unit;

    let a = Unit;
    let b = Unit;

    let _: Unit = a.merge(b).unwrap();
}

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_tuple() {
    #[derive(Default, Merge)]
    struct MyType(Merged, Merged);

    let a = MyType::default();
    let b = MyType::default();

    let c = a.merge(b).unwrap();

    assert!(c.0.0);
    assert!(c.1.0);
}

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_named() {
    #[derive(Default, Merge)]
    struct MyType {
        a: Merged,
        b: Merged,
        c: Merged,
    }

    let a = MyType::default();
    let b = MyType::default();

    let c = a.merge(b).unwrap();

    assert!(c.a.0);
    assert!(c.b.0);
    assert!(c.c.0);
}

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_rename() {
    use alloc::string::ToString;

    #[derive(Debug, Default, Merge)]
    struct MyType(#[merge(rename = "foo")] i32);

    let a = MyType::default();
    let b = MyType::default();

    let err = a.merge(b).unwrap_err();

    let mut iter = err.value.iter().map(|x| x.to_string());
    assert_eq!(iter.next().as_deref(), Some("foo"));
}

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_skip() {
    #[derive(Default, Merge)]
    struct MyType {
        #[merge(skip)]
        a: Merged,
        b: Merged,
        c: Merged,
    }

    let a = MyType::default();
    let b = MyType::default();

    let merged = a.merge(b).unwrap();

    assert!(!merged.a.0);
    assert!(merged.b.0);
    assert!(merged.c.0);
}

#[test]
#[cfg(feature = "derive")]
fn test_derive_merge_with() {
    mod custom {
        use super::*;

        pub fn merge(a: i32, b: i32) -> Result<i32, Error> {
            Ok(a + b)
        }

        pub fn merge_ref(a: &mut i32, b: i32) -> Result<(), Error> {
            *a += b;
            Ok(())
        }

        pub mod nested {
            pub use super::{merge, merge_ref};
        }
    }

    #[derive(Merge)]
    struct MyType {
        #[merge(with = custom)]
        a: i32,
        #[merge(with = custom::nested)]
        b: i32,
    }

    let a = MyType { a: 5, b: 12 };
    let b = MyType { a: -2, b: 42 };

    let merged = a.merge(b).unwrap();

    assert_eq!(merged.a, 3);
    assert_eq!(merged.b, 54);
}
