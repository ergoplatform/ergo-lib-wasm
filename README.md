# ergo-lib-wasm

WASM and JS/TS bindings for [`sigma-rust`](https://github.com/ergoplatform/sigma-rust).

We are currently working on migrating from the bindings/packages located [here](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm) and performing the first major upgrade of `ergo-lib-wasm`, you can see our progress [below](#ergo-lib-wasm).

## Repository setup

### ergo-lib-wasm

This is the main wasm crate providing bindings to `sigma-rust` and is published to NPM under the [`@ergoplatform/ergo-lib-wasm`](https://www.npmjs.com/package/@ergoplatform/ergo-lib-wasm) package.

The following functionality has been migrated:

| Description | Migrated? |
| --- |:---:|
| Merkle tree | ✅
| NiPoPow | ✅
| Addresses | ✅
| ErgoTree | ✅

### authenticated-avl-tree

Low level AVL+ tree library for interacting with trees off-chain.

### scorex-buffer

Cursor buffers implementing ZigZag/VLQ encoding as used for serialization on Ergo.

### Internal Crates

This repository contains a few utility crates that are used throughout the repository but are not published.

| Crate | Description |
| --- | --- |
| [`ergo-wasm-derive`](packages/ergo-wasm-derive) | Macros providing workarounds to [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) limitations. I.e working with arrays of WASM structs |
| [`ergo-lib-utils`](packages/ergo-lib-utils) | Common utility functions and traits |
