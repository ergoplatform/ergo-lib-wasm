# @ergoplatform/merkle-tree

## 0.1.1

### Patch Changes

- 694ec95: `BatchMerkleProof` json conversion now uses serde_json & strings

## 0.1.0

### Minor Changes

- d7c9466: Migrate `merkle-tree` functionality from sigma-rust.

  Adds the following:

  - MerkleProof
  - LevelNode
  - BatchMerkleProof

### Patch Changes

- cf05680: accept level nodes as a constructor param on `MerkleProof`
