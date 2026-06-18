//! [`File`] error.
//!
//! [`File`]: super::File

use std::fmt;
use std::path::PathBuf;
use std::string::ToString;

use crate::evaluator::dfs;

///////////////////////////////////////////////////////////////////////////////

/// Error for the [`File`] evaluator.
///
/// [`File`]: super::File
#[derive(Debug)]
#[non_exhaustive]
pub struct Error {
    /// Module trace.
    pub trace: dfs::Trace<PathBuf>,

    /// Evaluation error.
    pub error: dfs::Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            dfs::Error::Merge(ref e) => {
                e.message(f)?;
                writeln!(f, " while evaluating {}", e.field)?;
            }

            dfs::Error::Other(ref e) => {
                writeln!(f, "{}", e.to_string().trim_end())?;
            }
        }

        let mut trace = self.trace.iter().rev();
        if let Some(first) = trace.next() {
            writeln!(f)?;
            writeln!(f, "    in {}", first.display())?;
            for path in trace {
                writeln!(f, "  from {}", path.display())?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.error)
    }
}
