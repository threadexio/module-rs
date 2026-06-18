//! Generic [`Evaluator`] interface.
//!
//! Evaluator implementations:
//!
//! - [`Dfs`]

pub mod imports;
pub use self::imports::Imports;

pub mod dfs;
pub use self::dfs::Dfs;

#[cfg(feature = "std")]
pub mod metrics;
#[cfg(feature = "std")]
pub use self::metrics::Metrics;

#[cfg(feature = "std")]
pub mod acyclic;
#[cfg(feature = "std")]
pub use self::acyclic::Acyclic;

///////////////////////////////////////////////////////////////////////////////

/// An evaluator.
///
/// An evaluator is the main mechanism with which [`Merge`] is utilized. The
/// goal of the [`Evaluator`] is to [`Merge`] modules which may of may not
/// import other modules.
///
/// [`Merge`]: module::merge::Merge
pub trait Evaluator {
    /// Id of a module.
    ///
    /// This can be anything that can uniquely identify a module. For example,
    /// a filesystem path.
    type Id;

    /// The type of the module.
    type Module;

    /// Evaluation error.
    type Error;

    /// Check whether the evaluator has evaluated any modules.
    fn is_empty(&self) -> bool;

    /// Get the next module in evaluation order.
    ///
    /// Evaluation order is implementation-specific. An implementation might
    /// choose to evaluate modules in DFS, BFS or some other unspecified order.
    /// Returns [`None`] when there are no modules left to evaluate.
    fn next(&mut self) -> Option<Self::Id>;

    /// Evaluate `module` identified by `id` that imports the modules specified
    /// by `imports`.
    fn eval(
        &mut self,
        id: Self::Id,
        imports: Imports<Self::Id>,
        module: Self::Module,
    ) -> Result<(), Self::Error>;

    /// Destruct the evaluator and get the final value.
    ///
    /// Returns [`None`] when no modules have been evaluated.
    fn finish(self) -> Option<Self::Module>;
}
