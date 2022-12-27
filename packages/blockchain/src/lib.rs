use ergo_wasm_common::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BlockId(ergo_lib::ergo_chain_types::BlockId);

#[wasm_bindgen]
impl BlockId {
    pub fn from_hex(hex_str: &str) -> Result<BlockId, JsValue> {
        serde_json::from_str(hex_str)
            .map(BlockId)
            .map_err_js_value()
    }
}
