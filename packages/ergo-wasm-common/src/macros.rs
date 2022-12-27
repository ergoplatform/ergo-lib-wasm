pub use with_builtin_macros::with_builtin;

#[macro_export]
macro_rules! export_typescript_type {
    ($name:ident is $ts_tpe:literal) => {
        $crate::macros::with_builtin!(let $section = concat!("export type ", stringify!($name), " = ", $ts_tpe, ";") in {
            $crate::macros::with_builtin!(let $name_str = stringify!($name) in {
                #[wasm_bindgen(typescript_custom_section)]
                const _: &'static str = $section;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(typescript_type = $name_str)]
                    pub type $name;
                }
            });
        });
    };
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
