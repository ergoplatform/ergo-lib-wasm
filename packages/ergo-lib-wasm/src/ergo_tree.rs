//! ErgoTree

use crate::ir::constants::SConstant;
use crate::prelude::*;
use derive_more::{From, Into};
use ergo_lib::ergo_chain_types::Base16DecodedBytes;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

/// The root of ErgoScript IR. Serialized instances of this class are self sufficient and can be passed around.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct ErgoTree(ergo_lib::ergotree_ir::ergo_tree::ErgoTree);

#[wasm_bindgen]
impl ErgoTree {
    /// Decode from base16 encoded serialized ErgoTree
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex_str: &str) -> Result<ErgoTree, JsValue> {
        let bytes = Base16DecodedBytes::try_from(hex_str.to_string()).map_err_js_value()?;
        ErgoTree::from_bytes(bytes.0)
    }

    /// Returns Base16-encoded serialized bytes
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        self.0.to_base16_bytes().map_err_js_value()
    }

    /// Decode from encoded serialized ErgoTree
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<ErgoTree, JsValue> {
        ergo_lib::ergotree_ir::ergo_tree::ErgoTree::sigma_parse_bytes(&bytes)
            .map(ErgoTree)
            .map_err_js_value()
    }

    /// Returns serialized bytes or fails with error if ErgoTree cannot be serialized
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.0.sigma_serialize_bytes().map_err_js_value()
    }

    /// Returns number of constants stored in the Ergo Tree
    #[wasm_bindgen(getter, js_name = constantsLength)]
    pub fn constants_len(&self) -> Result<usize, JsValue> {
        self.0.constants_len().map_err_js_value()
    }

    /// Returns constant with given index (as stored in serialized ErgoTree)
    /// or None if index is out of bounds
    /// or error if constants parsing were failed
    #[wasm_bindgen(js_name = getConstant)]
    pub fn get_constant(&self, index: usize) -> Result<Option<SConstant>, JsValue> {
        self.0
            .get_constant(index)
            .map(|opt| opt.map(SConstant::from))
            .map_err_js_value()
    }

    /// Consumes the calling ErgoTree and returns new ErgoTree with a new constant value
    /// for a given index in constants list (as stored in serialized ErgoTree), or an error.
    /// After the call the calling ErgoTree will be null.
    #[wasm_bindgen(js_name = withConstant)]
    pub fn with_constant(self, index: usize, constant: &SConstant) -> Result<ErgoTree, JsValue> {
        self.0
            .with_constant(index, constant.clone().into())
            .map(ErgoTree)
            .map_err_js_value()
    }

    /// Serialized proposition expression of SigmaProp type with
    /// ConstantPlaceholder nodes instead of Constant nodes
    #[wasm_bindgen(getter, js_name = templateBytes)]
    pub fn template_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.0.template_bytes().map_err_js_value()
    }
}
