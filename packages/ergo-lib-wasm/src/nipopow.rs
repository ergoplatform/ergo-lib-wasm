//! Bindings for NiPoPow

use derive_more::{From, Into};
use ergo_wasm_common::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{blockchain::BlockId, merkle_tree::BatchMerkleProof};

/// A structure representing NiPoPow proof.
#[wasm_bindgen]
#[derive(Debug, From, Into, Serialize, Deserialize)]
pub struct NipopowProof(ergo_lib::ergo_nipopow::NipopowProof);

#[wasm_bindgen]
impl NipopowProof {
    /// Check if this {@link NipopowProof} is deemed better than `other`
    /// using the algorithm described below.
    ///
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

impl_json_methods!(NipopowProof);

/// A verifier for PoPoW proofs. During its lifetime, it processes many proofs with the aim of
/// deducing at any given point what is the best (sub)chain rooted at the specified genesis.
#[wasm_bindgen]
#[derive(Debug, From, Into)]
pub struct NipopowVerifier(ergo_lib::ergo_nipopow::NipopowVerifier);

#[wasm_bindgen]
impl NipopowVerifier {
    /// Create new instance
    #[wasm_bindgen(constructor)]
    pub fn new(genesis_block_id: BlockId) -> Result<NipopowVerifier, JsValue> {
        let block_id = ergo_lib::ergo_chain_types::Digest32::try_from(genesis_block_id)
            .map(|d| ergo_lib::ergo_chain_types::BlockId(d))
            .map_err_js_value()?;

        Ok(ergo_lib::ergo_nipopow::NipopowVerifier::new(block_id).into())
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
    // /// Returns block header
    // pub fn header(&self) -> BlockHeader {
    //     self.0.header.clone().into()
    // }

    /// Returns interlinks for PoPowHeader
    #[wasm_bindgen(getter)]
    pub fn interlinks(&self) -> Result<JsValue, JsError> {
        serde_json::to_string(&self.0.interlinks)
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Returns interlinks proof [`crate::batchmerkleproof::BatchMerkleProof`]
    #[wasm_bindgen(getter, js_name = interlinksProof)]
    pub fn interlinks_proof(&self) -> BatchMerkleProof {
        self.0.interlinks_proof.clone().into()
    }

    /// Validates interlinks merkle root with compact merkle multiproof. See [`PoPowHeader::interlinks_proof`] for BatchMerkleProof access
    #[wasm_bindgen(js_name = checkInterlinksProof)]
    pub fn check_interlinks_proof(&self) -> bool {
        self.0.check_interlinks_proof()
    }

    /// Returns block height for Header
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.0.header.height
    }

    /// Returns Block ID for Header
    pub fn id(&self) -> BlockId {
        self.0.header.id.clone().to_string()
    }
}
