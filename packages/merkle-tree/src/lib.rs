extern crate alloc;
use ergo_wasm_derive::{TryFromJsValue, TryJsArrayToVec, TryVecToJsArray};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
#[repr(C)]
/// Node Side in Merkle Tree
pub enum NodeSide {
    /// Node is on the left side of the current level
    Left = 0,
    /// Node is on the right side of the current level
    Right = 1,
}

impl From<ergo_merkle_tree::NodeSide> for NodeSide {
    fn from(value: ergo_merkle_tree::NodeSide) -> Self {
        match value {
            ergo_merkle_tree::NodeSide::Left => Self::Left,
            ergo_merkle_tree::NodeSide::Right => Self::Right,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "LevelNode[]")]
    pub type LevelNodeArray;
}

/// A level node in a merkle proof
#[derive(TryFromJsValue, TryVecToJsArray, TryJsArrayToVec)]
#[ergo(array_type = "LevelNodeArray")]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct LevelNode(ergo_merkle_tree::LevelNode);

#[wasm_bindgen]
impl LevelNode {
    /// Creates a new LevelNode from a 32 byte hash and side that the node belongs on in the tree. Fails if the digest is not 32 bytes
    #[wasm_bindgen(constructor)]
    pub fn new(hash: &[u8], side: u8) -> Result<LevelNode, JsValue> {
        Ok(Self(ergo_merkle_tree::LevelNode::new(
            (*hash)
                .try_into()
                .map_err(|_| "Digest is not 32 bytes in size")?,
            side.try_into()?,
        )))
    }

    /// Returns the associated digest (hash) with this node. Returns an empty array if there's no hash
    #[wasm_bindgen(getter)]
    pub fn digest(&self) -> Vec<u8> {
        match &self.0.hash {
            Some(hash) => hash.0.to_vec(),
            None => vec![],
        }
    }

    /// Returns the associated side with this node (0 = Left, 1 = Right)
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> NodeSide {
        self.0.side.into()
    }
}

#[wasm_bindgen]
/// A MerkleProof type. Given leaf data and levels (bottom-upwards), the root hash can be computed and validated
pub struct MerkleProof(pub(crate) ergo_merkle_tree::MerkleProof);

#[wasm_bindgen]
impl MerkleProof {
    /// Creates a new merkle proof with given leaf data and level data (bottom-upwards)
    /// You can verify it against a Blakeb256 root hash by using [`Self::valid()`]
    /// Add a node by using [`Self::add_node()`]
    /// Each digest on the level must be exactly 32 bytes
    #[wasm_bindgen(constructor)]
    pub fn new(leaf_data: &[u8]) -> MerkleProof {
        Self(ergo_merkle_tree::MerkleProof::new(leaf_data, &[])) // There are issues with wasm when trying to pass an array of structs, so it's better to use add_node instead
    }

    pub fn test_arrays(js_levels: &LevelNodeArray) -> LevelNodeArray {
        let typed_array: Vec<LevelNode> = js_levels.try_as_vec().unwrap();
        typed_array.try_into_js_array().unwrap()
    }

    /// Adds a new node to the MerkleProof above the current nodes
    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, level: &LevelNode) {
        self.0.add_node(level.0.clone());
    }

    /// Validates the Merkle proof against the root hash
    #[wasm_bindgen(js_name = isValid)]
    pub fn valid(&self, expected_root: &[u8]) -> bool {
        self.0.valid(expected_root)
    }
}

/// BatchMerkleProof type to validate root hash for multiple nodes
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMerkleProof(ergo_merkle_tree::BatchMerkleProof);

#[wasm_bindgen]
impl BatchMerkleProof {
    /// Calculates root hash for [`BatchMerkleProof`] and compares it against expected root hash
    #[wasm_bindgen(js_name = isValid)]
    pub fn valid(&self, expected_root: &[u8]) -> bool {
        self.0.valid(expected_root)
    }

    /// Creates a new [`BatchMerkleProof`] from json representation
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: JsValue) -> Result<BatchMerkleProof, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::from_value(json)
    }

    /// Converts [`BatchMerkleProof`] to json representation
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self.0)
    }
}
