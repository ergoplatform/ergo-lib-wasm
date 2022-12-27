use derive_more::{From, Into};
use ergo_merkle_tree_wasm::BatchMerkleProof;
use ergo_wasm_common::MapJsValueErrorResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@ergoplatform/merkle-tree")]
extern "C" {
    #[wasm_bindgen(typescript_type = "MerkleProof", js_name = "MerkleProof")]
    pub type MerkleProofTest;

    #[wasm_bindgen(js_name = getProof)] // And here
    pub fn get_proof() -> MerkleProofTest;

    #[wasm_bindgen(js_name = otherGetProof)] // And here
    pub fn other_get_proof() -> MerkleProofTest;
}

/// A structure representing NiPoPow proof.
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct NipopowProof(ergo_lib::ergo_nipopow::NipopowProof);

#[wasm_bindgen]
impl NipopowProof {
    #[wasm_bindgen]
    pub fn test_proof() -> MerkleProofTest {
        get_proof()
    }

    /// Implementation of the ≥ algorithm from [`KMZ17`], see Algorithm 4
    ///
    /// [`KMZ17`]: https://fc20.ifca.ai/preproceedings/74.pdf
    #[wasm_bindgen(js_name = isBetterThan)]
    pub fn is_better_than(&self, other: &NipopowProof) -> Result<bool, JsValue> {
        self.0.is_better_than(&other.0).map_err_js_value()
    }

    /// Get suffix head
    #[wasm_bindgen(getter, js_name = suffixHead)]
    pub fn suffix_head(&self) -> PoPowHeader {
        self.0.suffix_head.clone().into()
    }

    /// Converts the instance to a JSON object
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self.0)
    }

    /// Parse from JSON
    /// supports Ergo Node/Explorer API and box values and token amount encoded as strings
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: JsValue) -> Result<NipopowProof, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::from_value::<ergo_lib::ergo_nipopow::NipopowProof>(json).map(Self::from)
    }
}

/// PoPowHeader structure. Represents the block header and unpacked interlinks
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct PoPowHeader(ergo_lib::ergo_nipopow::PoPowHeader);

#[wasm_bindgen]
impl PoPowHeader {
    /// Returns interlinks proof [`crate::batchmerkleproof::BatchMerkleProof`]
    pub fn interlinks_proof(&self) -> BatchMerkleProof {
        // self.0.interlinks_proof
        self.0.interlinks_proof.clone().into()
    }

    /// Validates interlinks merkle root with compact merkle multiproof. See [`PoPowHeader::interlinks_proof`] for BatchMerkleProof access
    pub fn check_interlinks_proof(&self) -> bool {
        self.0.check_interlinks_proof()
    }

    /// Returns block height for Header
    pub fn height(&self) -> u32 {
        self.0.header.height
    }
}
