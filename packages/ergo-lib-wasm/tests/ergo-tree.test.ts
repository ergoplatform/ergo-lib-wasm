import { ErgoTree, SInt, SLong } from "../";

describe("ErgoTree", () => {
  let tree: ErgoTree;

  beforeEach(() => {
    tree = ErgoTree.fromHex(
      "100204a00b08cd021dde34603426402615658f1d970cfa7c7bd92ac81a8b16eeebff264d59ce4604ea02d192a39a8cc7a70173007301"
    );
  });

  describe("constantsLength", () => {
    it("should return correct length", () => {
      expect(tree.constantsLength).toBe(2);
    });
  });
  describe("getConstant", () => {
    it("should return a constant by index", () => {
      const const1 = tree.getConstant(0);
      const const2 = tree.getConstant(1);

      expect(const1).toBeDefined();
      expect(const2).toBeDefined();
    });
    it("should return undefined if the index is out-of-bounds", () => {
      const constant = tree.getConstant(3);

      expect(constant).toBeUndefined();
    });
  });
  describe("withConstant", () => {
    it("should create a new tree with the supplied constant", () => {
      const constant = new SInt(1337).intoConstant();
      const updatedTree = tree.withConstant(0, constant);
      const treeConstant = updatedTree.getConstant(0)!;

      // TODO: need to be able to convert treeConstant to JS value
      expect(true).toBe(true);
    });
    it("should throw when inserting constant at an out-of-bounds index", () => {
      const constant = new SInt(1337).intoConstant();

      expect(() => tree.withConstant(3, constant)).toThrow();
    });
    it("should throw when replacing constant with incorrect type", () => {
      const constant = new SLong(BigInt("112421411")).intoConstant();

      expect(() => tree.withConstant(0, constant)).toThrow();
    });
  });
});
