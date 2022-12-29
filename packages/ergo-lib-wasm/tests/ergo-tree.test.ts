import { ErgoTree } from "../";

describe("ErgoTree", () => {
  describe("constLength", () => {
    it("should return correct length", () => {
      const tree = ErgoTree.fromHex(
        "100204a00b08cd021dde34603426402615658f1d970cfa7c7bd92ac81a8b16eeebff264d59ce4604ea02d192a39a8cc7a70173007301"
      );

      expect(tree.constLength).toBe(2);
    });
  });
});
