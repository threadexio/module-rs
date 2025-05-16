//! Strings concatenated with `\n`.
//!
//! See: [`Lines`].

use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use core::ops::{Deref, DerefMut};

use alloc::string::String;

use super::prelude::*;

/// Strings concatenated with `\n`.
///
/// # serde
///
/// This type deserializes like [`String`].
#[derive(Default, PartialEq, Eq, Hash)]
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

impl Borrow<String> for Lines {
    #[inline]
    fn borrow(&self) -> &String {
        &self.content
    }
}

impl BorrowMut<String> for Lines {
    #[inline]
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.content
    }
}

impl AsRef<String> for Lines {
    #[inline]
    fn as_ref(&self) -> &String {
        &self.content
    }
}

impl AsMut<String> for Lines {
    #[inline]
    fn as_mut(&mut self) -> &mut String {
        &mut self.content
    }
}

impl Deref for Lines {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

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
