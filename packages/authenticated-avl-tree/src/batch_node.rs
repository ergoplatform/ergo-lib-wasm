use derive_more::{From, Into};
use scorex_crypto_avltree::batch_node::{AVLTree as NativeAVLTree, Node, NodeHeader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct AVLTree(NativeAVLTree);

#[wasm_bindgen]
impl AVLTree {
    #[wasm_bindgen(constructor)]
    pub fn new(key_length: usize, value_length: Option<usize>) -> AVLTree {
        AVLTree(NativeAVLTree::new(
            |digest| Node::LabelOnly(NodeHeader::new(Some(*digest), None)),
            key_length,
            value_length,
        ))
    }
}
