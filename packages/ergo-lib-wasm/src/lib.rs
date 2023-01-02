pub mod address;
pub mod blockchain;
pub mod ergo_tree;
pub mod macros;
pub mod merkle_tree;
pub mod nipopow;
pub mod traits;

/// A module that exports commonly used types for this crate.
///
/// Typically used in glob form:
///
/// ```
/// use crate::prelude::*;
/// ```
pub mod prelude {
    pub use crate::traits::{MapJsValueErrorResult, TryJsArrayToVec, TryVecToJsArray};
    pub use crate::{impl_json_methods, impl_try_js_array_to_vec, impl_try_vec_to_js_array};
}
