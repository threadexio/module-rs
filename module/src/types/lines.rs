//! Strings concatenated with `\n`.
//!
//! See: [`Lines`].

use core::fmt;

use alloc::string::String;

use super::prelude::*;

/// Strings concatenated with `\n`.
///
/// # Example
///
/// ```rust
/// # use module::types::Lines;
/// # use module::merge::{Merge, MergeCell};
/// let mut cell = MergeCell::empty();
///
/// cell.merge(Lines::new("line1\nline2"));
/// cell.merge(Lines::new("line3\n"));
/// cell.merge(Lines::new("line4\n\nline5"));
///
/// let lines = cell.finish().unwrap();
///
/// assert_eq!(*lines, r#"line1
/// line2
/// line3
/// line4
///
/// line5"#);
/// ```
///
/// # serde
///
/// This type deserializes like [`String`].
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Lines {
    content: String,
}

impl Lines {
    /// Create a new [`Lines`] with `content`.
    pub fn new<C>(content: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            content: content.into(),
        }
    }

    /// Destruct this wrapper.
    pub fn into_string(self) -> String {
        self.content
    }
}

impl Merge for Lines {
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        const SEP: char = '\n';

        if !self.content.ends_with(SEP) {
            self.content.push(SEP);
        }

        self.content.push_str(&other.content);

        Ok(())
    }
}

impl fmt::Debug for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.content.lines()).finish()
    }
}

impl From<String> for Lines {
    #[inline]
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<Lines> for String {
    #[inline]
    fn from(lines: Lines) -> Self {
        lines.into_string()
    }
}

impl_borrow!(Lines => str { .content });
impl_as_ref!(Lines => str { .content });
impl_wrapper!(Lines => String { .content });

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::Deserialize;
    use serde::de::Deserializer;

    impl<'de> Deserialize<'de> for Lines {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <String as Deserialize>::deserialize(deserializer).map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = Lines::new("line1");
        let b = Lines::new("line2");

        let merged = a.merge(b).unwrap();
        assert_eq!(*merged, "line1\nline2");
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let x: Lines = serde_json::from_str("\"test1\\ntest2\"").unwrap();
        assert_eq!(*x, "test1\ntest2");
    }
}
