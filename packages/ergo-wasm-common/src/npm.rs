//! Provide declarations so classes & functions can be imported and used via NPM packages.
//!
//! This is preferable to adding a dependency on another wasm binding crate at the rust level
//! as doing so causes the dependencies binding code to be generated in the dependents wasm bindings as well
//! leading to a lot of code duplication.
//!
//! Each `npm_import_*` macro includes a `typescript_custom_section` that is used to provide
//! the NPM modules types in consumers generated `.d.ts` files so the types can be resolved.
//!
//! Macros also define the types exported by the NPM package so they can be used
//! at the rust level without code duplication (because we're actually calling the JS functions).
//!
//! This is our solution to the `wasm-bindgen` limitation around using NPM packages: https://github.com/rustwasm/wasm-bindgen/issues/2771

/// Defines types & functions exported by the `@ergoplatform/merkle-tree` NPM package
#[macro_export]
macro_rules! npm_import_merkle_tree {
    () => {
        #[wasm_bindgen(typescript_custom_section)]
        const _: &'static str =
            r#"import type {MerkleProof, BatchMerkleProof} from "@ergoplatform/merkle-tree";"#;

        #[wasm_bindgen(module = "@ergoplatform/merkle-tree")]
        extern "C" {
            #[wasm_bindgen]
            pub type MerkleProof;

            #[wasm_bindgen(typescript_type = "BatchMerkleProof")]
            pub type BatchMerkleProof;

            #[wasm_bindgen(static_method_of = BatchMerkleProof, js_name = fromJSON)]
            fn from_json(json: String) -> BatchMerkleProof;
        }
    };
}

/// Defines types & functions exported by the `@ergoplatform/blockchain` NPM package
#[macro_export]
macro_rules! npm_import_blockchain {
    () => {
        #[wasm_bindgen(typescript_custom_section)]
        const _: &'static str = r#"import type {BlockId} from "@ergoplatform/blockchain";"#;

        #[wasm_bindgen(module = "@ergoplatform/blockchain")]
        extern "C" {
            #[wasm_bindgen]
            pub type BlockId;

            #[wasm_bindgen(static_method_of = BlockId, js_name = fromHex)]
            fn from_hex(hex: String) -> BlockId;
        }
    };
}

#[macro_export]
macro_rules! npm_import {
    ("@ergoplatform/merkle-tree") => {
        $crate::npm_import_merkle_tree!();
    };

    ("@ergoplatform/blockchain") => {
        $crate::npm_import_blockchain!();
    };
}
