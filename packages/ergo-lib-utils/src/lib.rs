use js_sys::{Array, Function, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// Used to lower a `JsValue` on the rust side to a concrete rust struct
/// that has been binded with `wasm_bindgen`.
///
/// `TryFromJsValue` derive macro defines the `__getClassname` function
/// to enable this conversion.
pub fn extract_classname(value: &JsValue) -> Result<String, JsValue> {
    let classname = Reflect::get(value, &JsValue::from("__getClassname"))
        .map_err(|_| JsValue::from_str("failed to lookup __getClassname property"))?;

    if classname.is_undefined() {
        return Err("__getClassname undefined".into());
    }

    let classname_func = classname.dyn_into::<Function>().map_err(|err| {
        JsValue::from_str(format!("__getClassname is not a function, {:?}", err).as_str())
    })?;

    Reflect::apply(&classname_func, value, &Array::new())
        .ok()
        .and_then(|v| v.as_string())
        .ok_or_else(|| JsValue::from_str("Failed to get classname"))
}

pub trait MapJsValueErrorResult<T> {
    fn map_err_js_value(self) -> Result<T, JsValue>;
}

impl<T, E> MapJsValueErrorResult<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn map_err_js_value(self) -> Result<T, JsValue> {
        self.map_err(|e| JsValue::from_str(format!("js error occurred: {:?}", e).as_str()))
    }
}

#[macro_export]
macro_rules! impl_set_console_panic_hook_fn {
    () => {
        /// Set a panic hook that provides richer debug/stack trace information
        /// on errors that occur in WASM code.
        ///
        /// Call this function in the entrypoint of your application.
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = "__setConsolePanicHook")]
        pub fn set_console_panic_hook() {
            console_error_panic_hook::set_once()
        }
    };
}
