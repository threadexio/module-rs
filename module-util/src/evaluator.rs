//! Generic module [`Evaluator`].

use core::fmt::{self, Display};
use core::iter::FusedIterator;

use alloc::collections::linked_list::{self, LinkedList};
use alloc::vec::Vec;

use module::Merge;
use module::merge::error::Modules;

use crate::Result;

///////////////////////////////////////////////////////////////////////////////

trait LinkedListExt<T> {
    fn append_front(&mut self, other: &mut Self);
}

impl<T> LinkedListExt<T> for LinkedList<T> {
    fn append_front(&mut self, other: &mut Self) {
        other.append(self);
        self.append(other);
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Instruction<T> {
    Enter(T),
    Exit,
    Eval(T),
}

impl<T> Instruction<T> {
    fn as_eval(&self) -> Option<&T> {
        if let Self::Eval(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_eval_mut(&mut self) -> Option<&mut T> {
        if let Self::Eval(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_eval(self) -> Option<T> {
        if let Self::Eval(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A list of imports.
///
/// # serde
///
/// This type deserializes as a "sequence".
///
/// See: [serde data model](https://serde.rs/data-model.html).
#[derive(Clone)]
pub struct Imports<T>(LinkedList<Instruction<T>>);

impl<T> fmt::Debug for Imports<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|x| {
                x.as_eval()
                    .expect("Imports should only have Eval instructions")
            }))
            .finish()
    }
}

impl<T> FromIterator<T> for Imports<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let inner = iter.into_iter().map(Instruction::Eval).collect();
        Self(inner)
    }
}

impl<T> From<T> for Imports<T> {
    fn from(value: T) -> Self {
        [value].into_iter().collect()
    }
}

impl<T> Default for Imports<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Imports<T> {
    /// Create an empty import list.
    pub const fn empty() -> Self {
        Self(LinkedList::new())
    }

    /// Get the number of imports in this list.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the import list is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add `import` to the list.
    pub fn push(&mut self, import: T) {
        self.0.push_back(Instruction::Eval(import));
    }

    /// Move over all imports from `imports` to `self`.
    ///
    /// This function is equivalent to [`push`]ing each item of `imports` to
    /// `self`.
    ///
    /// [`push`]: Imports::push
    pub fn append(&mut self, imports: &mut Self) {
        self.0.append(&mut imports.0);
    }

    /// Create an iterator over the imports of `self`.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.0.iter())
    }

    /// Create a mutable iterator over the imports of `self`.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.0.iter_mut())
    }
}

impl<'a, T> IntoIterator for &'a Imports<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Imports<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for Imports<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// An iterator over [`Imports`].
///
/// Created by [`Imports::iter`].
#[expect(missing_debug_implementations)]
pub struct Iter<'a, T>(linked_list::Iter<'a, Instruction<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| {
            x.as_eval()
                .expect("Imports should only have Eval instructions")
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| {
            x.as_eval()
                .expect("Imports should only have Eval instructions")
        })
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

/// A mutable iterator over [`Imports`].
///
/// Created by [`Imports::iter_mut`].
#[expect(missing_debug_implementations)]
pub struct IterMut<'a, T>(linked_list::IterMut<'a, Instruction<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| {
            x.as_eval_mut()
                .expect("Imports should only have Eval instructions")
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| {
            x.as_eval_mut()
                .expect("Imports should only have Eval instructions")
        })
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

/// Owning iterator over [`Imports`].
///
/// Created by [`Imports::into_iter`].
#[expect(missing_debug_implementations)]
pub struct IntoIter<T>(linked_list::IntoIter<Instruction<T>>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| {
            x.into_eval()
                .expect("Imports should only have Eval instructions")
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| {
            x.into_eval()
                .expect("Imports should only have Eval instructions")
        })
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use core::marker::PhantomData;

    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

    impl<'de, T> Deserialize<'de> for Imports<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct ImportsVisitor<T>(PhantomData<T>);

            impl<'de, T> Visitor<'de> for ImportsVisitor<T>
            where
                T: Deserialize<'de>,
            {
                type Value = Imports<T>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a sequence")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    let mut inner = LinkedList::new();

                    while let Some(e) = seq.next_element()? {
                        inner.push_back(Instruction::Eval(e));
                    }

                    Ok(Imports(inner))
                }
            }

            let visitor = ImportsVisitor(PhantomData);
            deserializer.deserialize_seq(visitor)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A generic evaluator that evaluates modules of type `Value`.
///
/// This evaluator can be used to decouple the way modules are read from the way
/// they are evaluated. It follows design paradigms of the ["sans-io" pattern].
///
/// This evaluator does not do anything on its own, it needs external code to
/// "drive" it. Something like an "event loop".
///
/// ```rust,no_run
/// # use module_util::evaluator::Evaluator;
/// let mut evaluator = Evaluator::new();
///
/// evaluator.import("module 1");
/// while let Some(this) = evaluator.next() {
///     println!("evaluating {this}...");
///
///     // Read the value of the module here from the filesystem, network,
///     // environment, etc.
///     let value: i32 = unimplemented!();
///
///     // Resolve what other modules this module wants to import. This is also
///     // specific to how you read a module. For example, it could be the
///     // top-level "imports" array of the JSON file.
///     let imports = unimplemented!();
///
///     evaluator.eval(this, imports, value).unwrap();
/// }
///
/// let value = evaluator.finish().unwrap();
/// println!("final value: {value}");
/// ```
///
/// This pattern allows you to implement whatever kind of module logic you want
/// inside the loop without modifying the evaluator.
///
/// ["sans-io" pattern]: https://sans-io.readthedocs.io/how-to-sans-io.html#how-to-write-i-o-free-protocol-implementations
#[derive(Debug, Clone)]
pub struct Evaluator<Ref, Value> {
    program: LinkedList<Instruction<Ref>>,
    trace: Vec<Ref>,
    value: Option<Value>,
}

impl<Ref, Value> Default for Evaluator<Ref, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ref, Value> Evaluator<Ref, Value> {
    /// Create a new evaluator.
    pub const fn new() -> Self {
        Self {
            program: LinkedList::new(),
            trace: Vec::new(),
            value: None,
        }
    }

    /// Create a new evaluator with an initial value.
    ///
    /// Calling [`finish`] on the returned [`Evaluator`] will never return
    /// [`None`].
    ///
    /// [`finish`]: Evaluator::finish
    pub const fn with(value: Value) -> Self {
        Self {
            program: LinkedList::new(),
            trace: Vec::new(),
            value: Some(value),
        }
    }

    /// Specify additional `imports` to be evaluated.
    ///
    /// This function is usually only used to provide the first module.
    pub fn import<I>(&mut self, imports: I)
    where
        I: Into<Imports<Ref>>,
    {
        self._import(imports.into())
    }

    fn _import(&mut self, imports: Imports<Ref>) {
        let Imports(mut imports) = imports;
        self.program.append(&mut imports);
    }

    /// Get the next module in the evaluation order.
    ///
    /// Modules are evaluated in DFS order.
    #[expect(clippy::should_implement_trait)]
    // This should not be implemented in [`Iterator::next`] because it does not
    // really make sense for this type to be used as an [`Iterator`].
    pub fn next(&mut self) -> Option<Ref> {
        loop {
            match self.program.pop_front()? {
                Instruction::Enter(x) => {
                    self.trace.push(x);
                }
                Instruction::Exit => {
                    self.trace.pop();
                }
                Instruction::Eval(x) => break Some(x),
            }
        }
    }

    /// Finish the evaluation and get the result.
    ///
    /// Returns [`Some`] with the final value or [`None`] if no module has been
    /// evaluated successfully and the [`Evaluator`] was created without an
    /// initial value.
    ///
    /// See: [`Evaluator::with`].
    pub fn finish(self) -> Option<Value> {
        self.value
    }
}

impl<Ref, Value> Evaluator<Ref, Value>
where
    Ref: Clone + Display + Send + Sync + 'static,
{
    /// Build the module trace for `this` module.
    ///
    /// This function can be used to attach a module trace to [`module::Error`]
    /// on any user-thrown errors during the evaluation loop.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use module_util::evaluator::Evaluator;
    /// # fn a() -> module_util::Result<i32> {
    /// use module::Error;
    ///
    /// let mut evaluator = Evaluator::new();
    ///
    /// evaluator.import("module 1");
    /// while let Some(this) = evaluator.next() {
    ///     // ...
    ///
    ///     if this == "module 2" {
    ///         return Err(Error::custom("module 2 is not allowed to be evaluated"))
    ///             .map_err(|mut e| {
    ///                 e.modules = evaluator.trace(this);
    ///                 e
    ///             });
    ///     }
    ///
    ///     // ...
    ///
    /// #    let imports = unimplemented!();
    /// #    let value: i32 = unimplemented!();
    ///     evaluator.eval(this, imports, value)?;
    /// }
    ///
    /// let value = evaluator.finish().unwrap();
    /// Ok(value)
    /// # }
    /// ```
    pub fn trace(&self, this: Ref) -> Modules {
        let mut modules = Modules::new();
        modules.push(this);
        self.trace
            .iter()
            .rev()
            .cloned()
            .for_each(|x| modules.push(x));

        modules
    }
}

impl<Ref, Value> Evaluator<Ref, Value>
where
    Ref: Clone + Display + Send + Sync + 'static,
    Value: Merge,
{
    /// Evaluate the module `this`.
    ///
    /// - `imports`: any modules `this` imports
    /// - `value`: the value of the `this` module
    ///
    /// Returns the result of the [`Merge`] operation on `value`. The returned
    /// error has the appropriate trace attached.
    ///
    /// See: [`Evaluator::trace`].
    pub fn eval(&mut self, this: Ref, imports: Imports<Ref>, value: Value) -> Result<()> {
        self.value = match self.value.take() {
            None => Some(value),
            Some(old) => match old.merge(value) {
                Ok(x) => Some(x),
                Err(mut e) => {
                    e.modules = self.trace(this);
                    return Err(e);
                }
            },
        };

        let Imports(mut imports) = imports;
        if !imports.is_empty() {
            imports.push_front(Instruction::Enter(this));
            imports.push_back(Instruction::Exit);
            self.program.append_front(&mut imports);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use module::Error;
    use module::types::NoMerge;

    use super::*;

    fn eval<T>(initial: &'static str, modules: &[(&str, &[&'static str], T)]) -> Result<T>
    where
        T: Clone + Merge,
    {
        let mut x = Evaluator::new();

        x.import(initial);
        while let Some(this) = x.next() {
            let Some((_, imports, value)) = modules.iter().find(|(name, _, _)| *name == this)
            else {
                return Err(Error::custom("no such module")).map_err(|mut e| {
                    e.modules = x.trace(this);
                    e
                });
            };

            let imports = imports.iter().copied().collect();
            let value = value.clone();

            x.eval(this, imports, value)?;
        }

        let value = x.finish().unwrap();
        Ok(value)
    }

    #[test]
    fn test_module_trace() {
        let err = eval(
            "module 1",
            &[
                ("module 1", &["module 2"], Some(NoMerge(()))),
                ("module 2", &["module 3", "module 4"], None),
                ("module 3", &[], None),
                ("module 4", &["module 5"], None),
                ("module 5", &["module 6"], None),
                ("module 6", &[], Some(NoMerge(()))),
            ],
        )
        .unwrap_err();

        let trace: Vec<_> = err.modules.iter().map(|x| x.to_string()).collect();

        assert_eq!(
            trace,
            &["module 1", "module 2", "module 4", "module 5", "module 6"]
        );
    }

    #[test]
    fn test_eval_order() {
        // If the order is correct (DFS), then this should reach the
        // "non-existent" module first and error there instead of erroring on
        // "module 6" because of the collision.
        let err = eval(
            "module 1",
            &[
                ("module 1", &["module 2", "module 3"], Some(NoMerge(()))),
                ("module 2", &["module 4"], None),
                ("module 4", &["module 5"], None),
                ("module 5", &["non-existent"], None),
                ("module 3", &["module 6"], None),
                ("module 6", &[], Some(NoMerge(()))),
            ],
        )
        .unwrap_err();

        assert!(!err.kind.is_collision());
    }
}
