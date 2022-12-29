use crate::declarations::HexString;
use crate::prelude::*;
use derive_more::{From, Into};
use ergo_lib::ergo_chain_types::Header;
use ergo_wasm_derive::TryFromJsValue;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use tsify::declare;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

#[declare]
#[allow(dead_code)]
type BlockId = HexString;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BlockId")]
    pub type JsBlockId;

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
    pub fn id(&self) -> JsBlockId {
        JsString::from(self.0.id.to_string()).unchecked_into()
    }

    /// Get transactions root
    #[wasm_bindgen(getter, js_name = txRoot)]
    pub fn transactions_root(&self) -> Vec<u8> {
        self.0.transaction_root.into()
    }
}

impl_json_methods!(BlockHeader);
