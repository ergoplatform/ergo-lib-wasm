use crate::prelude::*;
use derive_more::{From, Into};
use ergo_wasm_derive::TryFromJsValue;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, From, Into, Serialize, Deserialize)]
pub struct ErgoBox(ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox);

impl_json_methods!(ErgoBox);
