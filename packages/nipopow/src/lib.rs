use derive_more::{From, Into};
use ergo_wasm_common::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

npm_import!("@ergoplatform/merkle-tree");
npm_import!("@ergoplatform/blockchain");

/// A structure representing NiPoPow proof.
#[wasm_bindgen]
#[derive(Debug, From, Into, Serialize, Deserialize)]
pub struct NipopowProof(ergo_lib::ergo_nipopow::NipopowProof);

impl_json_methods!(NipopowProof);

#[wasm_bindgen]
impl NipopowProof {
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
}

/// A verifier for PoPoW proofs. During its lifetime, it processes many proofs with the aim of
/// deducing at any given point what is the best (sub)chain rooted at the specified genesis.
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct NipopowVerifier(ergo_lib::ergo_nipopow::NipopowVerifier);

#[wasm_bindgen]
impl NipopowVerifier {
    /// Create new instance
    #[wasm_bindgen(constructor)]
    pub fn new(genesis_block_id: &BlockId) -> NipopowVerifier {
        todo!()
        // ergo_lib::ergo_nipopow::NipopowVerifier::new(genesis_block_id.clone()).into()
    }

    /// Return best proof
    #[wasm_bindgen(getter, js_name = bestProof)]
    pub fn best_proof(&self) -> Option<NipopowProof> {
        self.0.best_proof().map(NipopowProof)
    }

    // /// Returns chain of `BlockHeader`s from the best proof.
    // pub fn best_chain(&self) -> BlockHeaders {
    //     BlockHeaders(self.0.best_chain().into_iter().map(|h| h.into()).collect())
    // }

    /// Process given proof
    pub fn process(&mut self, new_proof: NipopowProof) -> Result<(), JsValue> {
        self.0.process(new_proof.0).map_err_js_value()
    }
}

/// PoPowHeader structure. Represents the block header and unpacked interlinks
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct PoPowHeader(ergo_lib::ergo_nipopow::PoPowHeader);

#[wasm_bindgen]
impl PoPowHeader {
    pub fn interlinks_proof(&self) -> Result<BatchMerkleProof, JsValue> {
        Ok(BatchMerkleProof::from_json(
            serde_json::to_string(&self.0.interlinks_proof).map_err_js_value()?,
        ))
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
