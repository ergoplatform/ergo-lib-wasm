import { SColl, SByte, SInt } from "../../";

describe("Constants", () => {
  describe("Functional parity with library v0.x.x", () => {
    describe("Constant.from_byte_array(new Uint8Array([3, 2, 1]))", () => {
      it("should serialize to the same value", () => {
        const constant = new SColl([
          new SByte(3),
          new SByte(2),
          new SByte(1),
        ]).intoConstant();

        expect(constant.toHex()).toBe("0e03030201");
      });
    });
    describe("ergo.Constant.from_i32_array([4, 1, 2])", () => {
      it("should serialize to the same value", () => {
        const constant = new SColl([
          new SInt(4),
          new SInt(1),
          new SInt(2),
        ]).intoConstant();

        expect(constant.toHex()).toBe("1003080204");
      });
    });
    describe("Constant.from_coll_coll_byte([new Uint8Array([4, 1, 2, 3]), new Uint8Array([1, 2, 3])])", () => {
      it("should serialize to the same value", () => {
        const constant = new SColl([
          new SColl([new SByte(4), new SByte(1), new SByte(2), new SByte(3)]),
          new SColl([new SByte(1), new SByte(2), new SByte(3)]),
        ]).intoConstant();

        expect(constant.toHex()).toBe("1a02040401020303010203");
      });
    });
    describe("Constant.from_i32_array([4, 1, 2, 5, 0, 2, 8])", () => {
      it("should serialize to the same value", () => {
        const constant = new SColl([
          new SInt(4),
          new SInt(1),
          new SInt(2),
          new SInt(5),
          new SInt(0),
          new SInt(2),
          new SInt(8),
        ]).intoConstant();

        expect(constant.toHex()).toBe("10070802040a000410");
      });
    });
  });
});
