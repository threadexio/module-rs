//! [`Dfs`] evaluation.

use alloc::collections::{LinkedList, VecDeque};
use alloc::vec::Vec;

use module::merge::Merge;

use super::{Evaluator, Imports};

pub mod error;
pub use self::error::Error;

pub mod trace;
pub use self::trace::Trace;

///////////////////////////////////////////////////////////////////////////////

/// An [`Evaluator`] that visits modules in DFS order.
#[derive(Debug)]
pub struct Dfs<I, M> {
    imports: LinkedList<VecDeque<I>>,
    value: Option<M>,

    /// Module trace.
    pub trace: Trace<I>,
}

impl<I, M> Dfs<I, M> {
    /// Create a new [`Dfs`] evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            imports: LinkedList::new(),
            value: None,
            trace: Trace::empty(),
        }
    }
}

impl<I, M> Default for Dfs<I, M> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<I, M> Evaluator for Dfs<I, M>
where
    M: Merge,
{
    type Id = I;
    type Module = M;
    type Error = Error;

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn next(&mut self) -> Option<Self::Id> {
        loop {
            let x = self.imports.front_mut()?;

            match x.pop_front() {
                Some(x) => break Some(x),
                None => {
                    let _ = self.trace.pop();
                    let _ = self.imports.pop_front();
                    continue;
                }
            }
        }
    }

    fn eval(
        &mut self,
        id: Self::Id,
        imports: Imports<Self::Id>,
        module: Self::Module,
    ) -> Result<(), Self::Error> {
        self.trace.push(id);
        self.imports.push_front(VecDeque::from(Vec::from(imports)));
        self.value.merge_ref(Some(module)).map_err(Error::Merge)?;
        Ok(())
    }

    fn finish(self) -> Option<Self::Module> {
        self.value
    }
}
