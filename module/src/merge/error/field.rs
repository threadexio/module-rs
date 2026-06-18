//! Field names.

use core::convert::Infallible;
use core::fmt;
use core::iter::FusedIterator;
use core::str;
use core::str::FromStr;

use alloc::string::String;

///////////////////////////////////////////////////////////////////////////////

/// Name of a mergeable field.
///
/// This is used to indicate which field caused the error during the [`Merge`]
/// operation.
///
/// In the case of nested [`Merge`]s, the complete field name is denoted by a
/// dot-separated path of field names;
///
/// # Example
///
/// ```rust
/// # use module::merge::error::Field;
/// let mut field = Field::from_iter(["nested", "field", "test"]);
///
/// assert_eq!(format!("{field:?}"), "\"nested.field.test\"");
/// assert_eq!(format!("{field}"), "nested.field.test");
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Field(String);

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<A> FromIterator<A> for Field
where
    A: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut inner = String::new();

        for component in iter {
            push_back_component(&mut inner, component.as_ref());
        }

        Self(inner)
    }
}

impl FromStr for Field {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split('.').collect())
    }
}

impl Field {
    /// Create a new empty [`Field`];
    #[must_use]
    pub const fn empty() -> Self {
        Self(String::new())
    }

    /// Get the number of components in the field path.
    ///
    /// See: [`Field`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.chars().filter(|&c| c == '.').count() + 1
    }

    /// Check whether the field path has any components.
    ///
    /// See: [`Field`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the field path as a [`str`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Field;
    /// let field: Field = ["nested", "field", "name"].into_iter().collect();
    ///
    /// assert_eq!(field.as_str(), "nested.field.name");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get an iterator over all components of the field.
    ///
    /// See: [`Field`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Field;
    /// let field: Field = "nested.field.name".parse().unwrap();
    ///
    /// let mut components = field.components();
    /// assert_eq!(components.next(), Some("nested"));
    /// assert_eq!(components.next(), Some("field"));
    /// assert_eq!(components.next(), Some("name"));
    /// assert_eq!(components.next(), None);
    /// ```
    #[must_use]
    pub fn components(&self) -> Components<'_> {
        Components::new(self)
    }

    /// Add a component to the front of the path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Field;
    /// let mut field: Field = "nested.field.name".parse().unwrap();
    /// field.push_front("prefix");
    ///
    /// assert_eq!(field.as_str(), "prefix.nested.field.name");
    /// ```
    pub fn push_front(&mut self, component: &str) {
        push_front_component(&mut self.0, component);
    }

    /// Add a component to the back of the path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Field;
    /// let mut field: Field = "nested.field.name".parse().unwrap();
    /// field.push_back("suffix");
    ///
    /// assert_eq!(field.as_str(), "nested.field.name.suffix");
    /// ```
    pub fn push_back(&mut self, component: &str) {
        push_back_component(&mut self.0, component);
    }
}

impl From<Field> for String {
    fn from(f: Field) -> Self {
        f.0
    }
}

fn push_front_component(s: &mut String, component: &str) {
    if !s.is_empty() {
        s.reserve(component.len() + 1);
        s.insert(0, '.');
    }

    s.insert_str(0, component);
}

fn push_back_component(s: &mut String, component: &str) {
    if component.is_empty() {
        return;
    }

    if !s.is_empty() {
        s.reserve(1 + component.len());
        s.push('.');
    }

    s.push_str(component);
}

///////////////////////////////////////////////////////////////////////////////

/// Borrowing iterator over all components of the [`Field`].
///
/// See: [`Field::components`].
#[derive(Clone)]
pub struct Components<'a>(str::Split<'a, char>);

impl fmt::Debug for Components<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Components")
            .field(&fmt::from_fn(|f| {
                f.debug_list().entries(self.clone()).finish()
            }))
            .finish()
    }
}

impl<'a> Components<'a> {
    fn new(field: &'a Field) -> Self {
        Self(field.0.split('.'))
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some("") => None,
            Some(x) => Some(x),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Components<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0.next_back() {
            Some("") => None,
            Some(x) => Some(x),
            None => None,
        }
    }
}

impl FusedIterator for Components<'_> {}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_field_eq {
        ($field:expr, $components:expr, $str:expr ) => {{
            use ::alloc::vec::Vec;

            let field: Field = $field;
            let components: &[&str] = $components;
            let str: &str = $str;
            assert_eq!(field.components().collect::<Vec<_>>(), components);
            assert_eq!(field.as_str(), str);

            let mut reconstructed = Field::empty();
            for c in components {
                reconstructed.push_back(c);
            }
            assert_eq!(reconstructed, field);
        }};
    }

    #[test]
    fn test_simple() {
        assert_field_eq!("".parse().unwrap(), &[], "");

        assert_field_eq!("test".parse().unwrap(), &["test"], "test");

        assert_field_eq!(
            "test.field.name".parse().unwrap(),
            &["test", "field", "name"],
            "test.field.name"
        );
    }

    #[test]
    fn test_empty_component() {
        assert_field_eq!(".".parse().unwrap(), &[], "");
        assert_field_eq!("...".parse().unwrap(), &[], "");

        assert_field_eq!(
            "test..name".parse().unwrap(),
            &["test", "name"],
            "test.name"
        );

        assert_field_eq!(".test".parse().unwrap(), &["test"], "test");
        assert_field_eq!("...test".parse().unwrap(), &["test"], "test");

        assert_field_eq!("test.".parse().unwrap(), &["test"], "test");
        assert_field_eq!("test...".parse().unwrap(), &["test"], "test");
    }

    #[test]
    fn test_weird_component() {
        assert_field_eq!(
            " περιεργο κομματι ".parse().unwrap(),
            &[" περιεργο κομματι "],
            " περιεργο κομματι "
        );

        assert_field_eq!(
            "test. περιεργο κομματι .name".parse().unwrap(),
            &["test", " περιεργο κομματι ", "name"],
            "test. περιεργο κομματι .name"
        );
    }
}
