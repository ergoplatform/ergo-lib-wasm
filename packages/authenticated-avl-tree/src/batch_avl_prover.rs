use crate::batch_node::AVLTree;
use crate::operation::{Operation, TsOperationType};
use derive_more::{From, Into};
use ergo_avltree_rust::authenticated_tree_ops::AuthenticatedTreeOps;
use ergo_avltree_rust::batch_avl_prover::BatchAVLProver as NativeBatchAVLProver;
use ergo_lib_utils::MapJsValueErrorResult;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(From, Into)]
pub struct BatchAVLProver(NativeBatchAVLProver);

#[wasm_bindgen]
impl BatchAVLProver {
    #[wasm_bindgen(constructor)]
    pub fn new(tree: &AVLTree, collect_changed_nodes: bool) -> BatchAVLProver {
        BatchAVLProver(NativeBatchAVLProver::new(
            tree.clone().into(),
            collect_changed_nodes,
        ))
    }

    /// Perform a single operation on the underlying {@link AVLTree}.
    #[wasm_bindgen(js_name = performOneOperation)]
    pub fn perform_one_operation(&mut self, op_js: &TsOperationType) -> Result<JsValue, JsValue> {
        let operation: Operation = (*op_js).to_owned().try_into().map_err_js_value()?;
        let result = self
            .0
            .perform_one_operation(&operation.into())
            .map_err_js_value()?
            .map_or(JsValue::UNDEFINED, |bytes| {
                Uint8Array::from(bytes.to_vec().as_slice()).into()
            });
        Ok(result)
    }

    /// Generates the proof for all the operations performed (except the ones that failed)
    /// since the last {@link BatchAVLProver.generateProof} call.
    #[wasm_bindgen(js_name = generateProof)]
    pub fn generate_proof(&mut self) -> Box<[u8]> {
        self.0.generate_proof().to_vec().into_boxed_slice()
    }

    pub fn digest(&self) -> JsValue {
        self.0.digest().map_or(JsValue::UNDEFINED, |digest| {
            Uint8Array::from(digest.to_vec().as_slice()).into()
        })
    }
}
