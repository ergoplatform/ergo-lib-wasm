use wasm_bindgen::JsValue;

pub trait TryVecToJsArray {
    type ReturnType;

    fn try_into_js_array(self) -> Result<Self::ReturnType, ::wasm_bindgen::JsValue>;
    fn try_as_js_array(&self) -> Result<Self::ReturnType, ::wasm_bindgen::JsValue>;
}

pub trait TryJsArrayToVec {
    type ReturnType;

    fn try_to_vec(&self) -> Result<Vec<Self::ReturnType>, ::wasm_bindgen::JsValue>;
}

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
