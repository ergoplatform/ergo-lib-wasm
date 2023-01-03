use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};

/// Used to lower a `JsValue` on the rust side to a concrete rust struct
/// that has been binded with `wasm_bindgen`.
///
/// `TryFromJsValue` derive macro defines the `__getClassname` function
/// to enable this conversion.
pub(crate) fn extract_classname(value: &JsValue) -> Result<String, JsValue> {
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
