//! [`Evaluator`] adapter for preventing infinite evaluation due to cyclic imports.

use std::collections::HashSet;
use std::hash::Hash;

use super::{Evaluator, Imports};

///////////////////////////////////////////////////////////////////////////////

/// An [`Evaluator`] adapter that prevents infinite evaluation from cyclic
/// imports.
#[derive(Debug, Clone)]
pub struct Acyclic<E>
where
    E: Evaluator,
{
    evaluator: E,
    evaluated: HashSet<E::Id>,
}

impl<E> Acyclic<E>
where
    E: Evaluator,
{
    /// Create a new [`Acyclic`].
    pub fn new(evaluator: E) -> Self {
        Self {
            evaluator,
            evaluated: HashSet::new(),
        }
    }
}

impl<E> Default for Acyclic<E>
where
    E: Evaluator + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<E> Acyclic<E>
where
    E: Evaluator,
    E::Id: Eq + Hash,
{
    /// Check whether `id` was evaluated.
    ///
    /// A module is marked as "evaluated" when [`eval()`] succeeds on it.
    pub fn evaluated(&self, id: &E::Id) -> bool {
        self.evaluated.contains(id)
    }

    /// Get a reference to the inner evaluator.
    pub fn get(&self) -> &E {
        &self.evaluator
    }

    /// Get a mutable reference to the inner evaluator.
    pub fn get_mut(&mut self) -> &mut E {
        &mut self.evaluator
    }
}

impl<E> Evaluator for Acyclic<E>
where
    E: Evaluator,
    E::Id: Clone + Eq + Hash,
{
    type Id = E::Id;
    type Module = E::Module;
    type Error = E::Error;

    fn is_empty(&self) -> bool {
        self.evaluator.is_empty()
    }

    fn next(&mut self) -> Option<Self::Id> {
        loop {
            match self.evaluator.next() {
                Some(x) if self.evaluated(&x) => continue,
                Some(x) => break Some(x),
                None => break None,
            }
        }
    }

    fn eval(
        &mut self,
        id: Self::Id,
        imports: Imports<Self::Id>,
        module: Self::Module,
    ) -> Result<(), Self::Error> {
        if self.evaluated(&id) {
            return Ok(());
        }

        let r = self.evaluator.eval(id.clone(), imports, module);
        if r.is_ok() {
            self.evaluated.insert(id);
        }

        r
    }

    fn finish(self) -> Option<Self::Module> {
        self.evaluator.finish()
    }
}
