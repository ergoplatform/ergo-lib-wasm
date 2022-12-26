# ergo-lib-wasm

WASM and JS/TS bindings for [`sigma-rust`](https://github.com/ergoplatform/sigma-rust).

We are currently working on migrating from the monolithic bindings/packages located [here](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm), you can see our progress [below](#published-crates).

## Repository setup

### Published Crates

The following crates contain bindings to specific `sigma-rust` functionality and are published to NPM.

| Crate | Description | NPM | Migrated? |
| --- | --- | --- | --- |
| [`merkle-tree`](packages/merkle-tree) | Library for working with merkle trees | [`@ergoplatform/merkle-tree`](https://www.npmjs.com/package/@ergoplatform/merkle-tree) | ✅

### Internal Crates

This repository contains a few utility crates that are used throughout the monorepo but not published.

| Crate | Description |
| --- | --- |
| [`ergo-wasm-common`](packages/ergo-wasm-common) | Utilities/traits/etc commonly used in bindings crates |
| [`ergo-wasm-derive`](packages/ergo-wasm-derive) | Macros providing workarounds to [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) limitations. I.e working with arrays of WASM structs |
