# @ergoplatform/scorex-buffer

## 1.0.4

### Patch Changes

- a90590d: Fix missing `sideEffect` causing ESM module WASM to break

## 1.0.3

### Patch Changes

- ed3a0cb: update repository field in package.json

## 1.0.2

### Patch Changes

- 900eff6: add `__setConsolePanicHook` function to provide more detailed stack traces in the event of errors originating from WASM code

## 1.0.1

### Patch Changes

- 3c9a3a0: - release initial authenticated AVL tree package with prover support
  - use `ergo-lib-utils` helper package in `scorex-buffer` library

## 1.0.0

### Major Changes

- b6360b8: Adds ScorexReader and ScorexWriter for reading & writing to buffers using scorex networking encoding.

  Scorex using zigzag and VLQ encoding of data sent over the wire.
