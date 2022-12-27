pub(crate) mod npm;

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

/// Implement & bind toJSON and fromJSON methods for wasm structs
#[macro_export]
macro_rules! impl_json_methods {
    ($x:ident) => {
        #[wasm_bindgen]
        impl $x {
            /// Converts the instance to JSON
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> Result<String, ::wasm_bindgen::JsValue> {
                serde_json::to_string(&self.0).map_err_js_value()
            }

            /// Creates an instance from JSON
            #[wasm_bindgen(js_name = fromJSON)]
            pub fn from_json(json: &str) -> Result<$x, ::wasm_bindgen::JsValue> {
                serde_json::from_str(json).map_err_js_value()
            }
        }
    };
}

/// A module that imports commonly used types.
///
/// Typically used in glob form:
///
/// ```
/// use ergo_wasm_common::prelude::*;
/// ```
pub mod prelude {
    pub use crate::impl_json_methods;
    pub use crate::npm_import;
    pub use crate::MapJsValueErrorResult;
}
