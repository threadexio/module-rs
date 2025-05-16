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
