use std::io::Cursor;

use ergo_lib_utils::MapJsValueErrorResult;
use js_sys::Uint8Array;
use sigma_ser::vlq_encode::{ReadSigmaVlqExt, WriteSigmaVlqExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ScorexWriter(Vec<u8>);

#[wasm_bindgen]
impl ScorexWriter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ScorexWriter {
        ScorexWriter(vec![])
    }

    #[wasm_bindgen(js_name = putInt8)]
    pub fn put_i8(&mut self, v: i8) -> Result<(), JsValue> {
        self.0.put_i8(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putUint8)]
    pub fn put_u8(&mut self, v: u8) -> Result<(), JsValue> {
        self.0.put_u8(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putInt16)]
    pub fn put_i16(&mut self, v: i16) -> Result<(), JsValue> {
        self.0.put_i16(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putUint16)]
    pub fn put_u16(&mut self, v: u16) -> Result<(), JsValue> {
        self.0.put_u16(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putInt32)]
    pub fn put_i32(&mut self, v: i32) -> Result<(), JsValue> {
        self.0.put_i32(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putUint32)]
    pub fn put_u32(&mut self, v: u32) -> Result<(), JsValue> {
        self.0.put_u32(v).map_err_js_value()
    }

    #[wasm_bindgen(js_name = putInt64)]
    pub fn put_i64(&mut self, v: &js_sys::BigInt) -> Result<(), JsValue> {
        self.0
            .put_i64(v.clone().try_into().map_err_js_value()?)
            .map_err_js_value()
    }

    #[wasm_bindgen(js_name = putUint64)]
    pub fn put_u64(&mut self, v: &js_sys::BigInt) -> Result<(), JsValue> {
        self.0
            .put_u64(v.clone().try_into().map_err_js_value()?)
            .map_err_js_value()
    }

    #[wasm_bindgen(js_name = putString)]
    pub fn put_str(&mut self, v: &str) -> Result<(), JsValue> {
        self.0.put_short_string(v).map_err_js_value()
    }

    #[wasm_bindgen(getter)]
    pub fn buffer(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[wasm_bindgen]
pub struct ScorexReader(Cursor<Box<[u8]>>);

#[wasm_bindgen]
impl ScorexReader {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: &Uint8Array) -> ScorexReader {
        ScorexReader(Cursor::new(buf.to_vec().into_boxed_slice()))
    }

    #[wasm_bindgen(js_name = getInt8)]
    pub fn get_i8(&mut self) -> Result<i8, JsValue> {
        self.0.get_i8().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getUint8)]
    pub fn get_u8(&mut self) -> Result<u8, JsValue> {
        self.0.get_u8().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getInt16)]
    pub fn get_i16(&mut self) -> Result<i16, JsValue> {
        self.0.get_i16().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getUint16)]
    pub fn get_u16(&mut self) -> Result<u16, JsValue> {
        self.0.get_u16().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getInt32)]
    pub fn get_i32(&mut self) -> Result<i32, JsValue> {
        self.0.get_i32().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getUint32)]
    pub fn get_u32(&mut self) -> Result<u32, JsValue> {
        self.0.get_u32().map_err_js_value()
    }

    #[wasm_bindgen(js_name = getInt64)]
    pub fn get_i64(&mut self) -> Result<js_sys::BigInt, JsValue> {
        let v = self.0.get_i64().map_err_js_value()?;

        Ok(js_sys::BigInt::from(v))
    }

    #[wasm_bindgen(js_name = getUint64)]
    pub fn get_u64(&mut self) -> Result<js_sys::BigInt, JsValue> {
        let v = self.0.get_u64().map_err_js_value()?;

        Ok(js_sys::BigInt::from(v))
    }

    #[wasm_bindgen(js_name = getString)]
    pub fn get_str(&mut self) -> Result<String, JsValue> {
        self.0.get_short_string().map_err_js_value()
    }

    #[wasm_bindgen(getter)]
    pub fn buffer(&self) -> Vec<u8> {
        self.0.clone().into_inner().into_vec()
    }
}
