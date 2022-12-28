pub mod macros;

use wasm_bindgen::JsValue;

pub trait MapJsValueErrorResult<T> {
    fn map_err_js_value(self) -> Result<T, JsValue>;
}

impl<T, E> MapJsValueErrorResult<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn map_err_js_value(self) -> Result<T, JsValue> {
        self.map_err(|e| JsValue::from_str(format!("{:?}", e).as_str()))
    }
}

/// A module that exports commonly used types.
///
/// Typically used in glob form:
///
/// ```
/// use ergo_wasm_common::prelude::*;
/// ```
pub mod prelude {
    pub use crate::impl_json_methods;
    pub use crate::MapJsValueErrorResult;
}
