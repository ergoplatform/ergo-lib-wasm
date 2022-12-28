/// Implement & bind toJSON and fromJSON methods for wasm structs
#[macro_export]
macro_rules! impl_json_methods {
    ($x:ident) => {
        #[wasm_bindgen]
        impl $x {
            /// Converts the instance to a JSON string
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> Result<String, ::wasm_bindgen::JsValue> {
                $crate::MapJsValueErrorResult::map_err_js_value(serde_json::to_string(self))
            }

            /// Creates an instance from a JSON string
            #[wasm_bindgen(js_name = fromJSON)]
            pub fn from_json(json: &str) -> Result<$x, ::wasm_bindgen::JsValue> {
                $crate::MapJsValueErrorResult::map_err_js_value(serde_json::from_str(json))
            }
        }
    };
}
