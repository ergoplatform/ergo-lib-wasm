import {
  BLOCK_HEADER,
  MERKLE_PROOF,
  NIPOPOW_PROOF,
} from "./__testdata__/nipopow";
import {
  BlockHeader,
  BlockId,
  MerkleProof,
  NipopowProof,
  NipopowVerifier,
} from "..";

describe("NiPoPow", () => {
  describe("Basic SPV workflow", () => {
    it("should produce valid proofs using static test data", () => {
      const headerId = BlockId.fromHex(
        "d1366f762e46b7885496aaab0c42ec2950b0422d48aec3b91f45d4d0cdeb41e5"
      );
      const genesisBlockId = BlockId.fromHex(
        "b0244dfc267baca974a4caee06120321562784303a8a688976ae56170e4d175b"
      );
      const verifier = new NipopowVerifier(genesisBlockId);

      // NiPoPow proofs normally fetched from multiple peers and best chain selected
      // replacer func until this is merged/released: https://github.com/ergoplatform/sigma-rust/pull/668
      const jsonStr = JSON.stringify(NIPOPOW_PROOF, (key, val) => {
        if (key === "d") {
          return BigInt(val).toString();
        }

        return val;
      });

      const nipopowProof = NipopowProof.fromJSON(jsonStr);

      verifier.process(nipopowProof);

      const isBestProofHeaderEq =
        verifier.bestProof?.suffixHead.id.isEqual(headerId);

      expect(isBestProofHeaderEq).toBe(true);

      // merkle proof fetched from another peer not used to retrieve nipopow proofs
      const merkleProof = MerkleProof.fromJSON(JSON.stringify(MERKLE_PROOF));

      const header = BlockHeader.fromJSON(JSON.stringify(BLOCK_HEADER));
      expect(merkleProof.isValid(header.txRoot));
    });
  });
});
