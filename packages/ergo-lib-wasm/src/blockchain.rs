use crate::prelude::*;
use derive_more::{From, Into};
use ergo_lib::ergo_chain_types::Header;
use ergo_wasm_derive::TryFromJsValue;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct BlockId(pub(crate) ergo_lib::ergo_chain_types::BlockId);

#[wasm_bindgen]
impl BlockId {
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        Ok(self.0.to_string())
    }

    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<BlockId, JsValue> {
        ergo_lib::ergo_chain_types::Digest32::try_from(String::from(hex))
            .map(|d| BlockId(ergo_lib::ergo_chain_types::BlockId(d)))
            .map_err_js_value()
    }

    #[wasm_bindgen(js_name = isEqual)]
    pub fn is_equal(&self, other: &BlockId) -> bool {
        self.0 == other.0
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BlockHeader[]")]
    pub type JsBlockHeaderArray;
}

/// Block header
#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, From, Into, Serialize, Deserialize)]
pub struct BlockHeader(Header);

impl_try_vec_to_js_array!(vec of BlockHeader to JsBlockHeaderArray);

#[wasm_bindgen]
impl BlockHeader {
    /// Get Header's id
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> BlockId {
        self.0.id.into()
    }

    /// Get transactions root
    #[wasm_bindgen(getter, js_name = txRoot)]
    pub fn transactions_root(&self) -> Vec<u8> {
        self.0.transaction_root.into()
    }
}

impl_json_methods!(BlockHeader);
