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
