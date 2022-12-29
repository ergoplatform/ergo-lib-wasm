#[macro_export]
macro_rules! impl_try_js_array_to_vec {
    ($jsarr:ident to vec of $x:ident) => {
        impl $crate::traits::TryJsArrayToVec for &$jsarr {
            type ReturnType = $x;

            fn try_to_vec(&self) -> Result<Vec<Self::ReturnType>, ::wasm_bindgen::JsValue> {
                use ::wasm_bindgen::JsCast;
                use ::wasm_bindgen::JsValue;

                let js_array: &::js_sys::Array = self.dyn_ref().map_or_else(
                    || {
                        Err(JsValue::from_str(
                            "try_as_vec: argument wasn't an array type",
                        ))
                    },
                    |v| Ok(v),
                )?;
                let mut rust_vec =
                    Vec::<Self::ReturnType>::with_capacity(js_array.length() as usize);
                for js in js_array.iter() {
                    let elem = ::std::convert::TryFrom::try_from(&js)?;
                    rust_vec.push(elem);
                }

                Ok(rust_vec)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_vec_to_js_array {
    (vec of $x:ident to $jsarr:ident) => {
        impl $crate::traits::TryVecToJsArray for Vec<$x> {
            type ReturnType = $jsarr;

            fn try_into_js_array(self) -> Result<Self::ReturnType, ::wasm_bindgen::JsValue> {
                use ::wasm_bindgen::JsCast;

                Ok(self
                    .into_iter()
                    .map(::wasm_bindgen::JsValue::from)
                    .collect::<::js_sys::Array>()
                    .unchecked_into::<Self::ReturnType>())
            }

            fn try_as_js_array(&self) -> Result<Self::ReturnType, ::wasm_bindgen::JsValue> {
                use ::wasm_bindgen::JsCast;

                Ok(self
                    .iter()
                    .map(|f| ::wasm_bindgen::JsValue::from(f.clone()))
                    .collect::<::js_sys::Array>()
                    .unchecked_into::<Self::ReturnType>())
            }
        }
    };
}

/// Implement & bind toJSON and fromJSON methods for wasm structs
#[macro_export]
macro_rules! impl_json_methods {
    ($x:ident) => {
        #[wasm_bindgen]
        impl $x {
            /// Converts the instance to a JSON string
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> Result<String, ::wasm_bindgen::JsValue> {
                crate::traits::MapJsValueErrorResult::map_err_js_value(serde_json::to_string(self))
            }

            /// Creates an instance from a JSON string
            #[wasm_bindgen(js_name = fromJSON)]
            pub fn from_json(json: &str) -> Result<$x, ::wasm_bindgen::JsValue> {
                crate::traits::MapJsValueErrorResult::map_err_js_value(serde_json::from_str(json))
            }
        }
    };
}
