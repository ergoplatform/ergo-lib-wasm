use ergo_wasm_common::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

export_typescript_type!(HexString is "string");
export_typescript_type!(BlockIdA is "HexString");

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BlockId(ergo_lib::ergo_chain_types::BlockId);

#[wasm_bindgen]
impl BlockId {
    #[wasm_bindgen(js_name = "fromHex")]
    pub fn from_hex(hex_str: &HexString) -> Result<BlockId, JsValue> {
        let s = hex_str.as_string().unwrap();
        ergo_lib::ergo_chain_types::Digest32::try_from(s)
            .map(|d| BlockId(ergo_lib::ergo_chain_types::BlockId(d)))
            .map_err_js_value()
    }
}
