import { BlockId } from "../";

describe("BlockId", () => {
  describe("fromHex()", () => {
    it("should convert JSON to a BlockId instance", () => {
      const genesisId =
        "b0244dfc267baca974a4caee06120321562784303a8a688976ae56170e4d175b";

      expect(() => BlockId.fromHex(genesisId)).not.toThrow();
    });
  });

  describe("toJSON()", () => {});
});
