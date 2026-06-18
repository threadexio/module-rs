//! [`Evaluator`] adapter for collecting metrics.

use std::collections::HashSet;
use std::hash::Hash;

use super::{Evaluator, Imports};

///////////////////////////////////////////////////////////////////////////////

/// An [`Evaluator`] adapter that collects metrics during evaluation.
#[derive(Debug, Clone)]
pub struct Metrics<E>
where
    E: Evaluator,
{
    evaluator: E,

    /// Set of all evaluated modules.
    pub modules: HashSet<E::Id>,
}

impl<E> Metrics<E>
where
    E: Evaluator,
{
    /// Create a new [`Metrics`] adapter.
    pub fn new(evaluator: E) -> Self {
        Self {
            evaluator,
            modules: HashSet::new(),
        }
    }
}

impl<E> Default for Metrics<E>
where
    E: Evaluator + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<E> Metrics<E>
where
    E: Evaluator + Default,
{
    /// Get a reference to the inner evaluator.
    pub fn get(&self) -> &E {
        &self.evaluator
    }

    /// Get a mutable reference to the inner evaluator.
    pub fn get_mut(&mut self) -> &mut E {
        &mut self.evaluator
    }
}

impl<E> Evaluator for Metrics<E>
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
        self.evaluator.next()
    }

    fn eval(
        &mut self,
        id: Self::Id,
        imports: Imports<Self::Id>,
        module: Self::Module,
    ) -> Result<(), Self::Error> {
        self.modules.insert(id.clone());
        self.evaluator.eval(id, imports, module)
    }

    fn finish(self) -> Option<Self::Module> {
        self.evaluator.finish()
    }
}
