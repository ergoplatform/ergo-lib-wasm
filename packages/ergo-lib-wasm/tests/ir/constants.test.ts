import each from "jest-each";
import {
  SColl,
  SByte,
  SInt,
  SLong,
  SBigInt,
  SSigmaProp,
  SGroupElement,
  ErgoBox,
  SErgoBox,
  SConstant,
  SBoolean,
  SShort,
  SUnit,
  SType,
} from "../../";

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
    describe("i64 support", () => {
      it("should handle the max value of i64", () => {
        const constant = new SLong(BigInt("9223372036854775807")); // max i64 value

        expect(constant.value).toBe(BigInt("9223372036854775807"));
        expect(() => constant.intoConstant()).not.toThrow();
      });
    });
    describe("BigInt support", () => {
      it("should handle bigint constants", () => {
        const constant = new SBigInt(
          BigInt("92233720368547758071111111111111111111111111")
        );

        expect(constant.value).toBe(
          BigInt("92233720368547758071111111111111111111111111")
        );
        expect(() => constant.intoConstant()).not.toThrow();
      });
    });
    describe("SigmaProp support", () => {
      each([[true], [false]]).it(
        "should handle trivial bool: %s",
        (bool: boolean) => {
          const constant = SSigmaProp.fromBool(bool);

          expect(constant.value).toBe(bool);
          expect(() => constant.intoConstant()).not.toThrow();
        }
      );
      it("should handle complex propositions from JSON", () => {
        const prop = {
          op: "205", // OpCode::PROVE_DLOG
          h: "02d6b2141c21e4f337e9b065a031a6269fb5a49253094fc6243d38662eb765db00", // random EC point
        };
        const constant = SSigmaProp.fromJSON(
          JSON.stringify(prop)
        ).intoConstant();

        expect(constant).toBeDefined();
      });
    });
    describe("GroupElement support", () => {
      it("it should be creatable from hex string", () => {
        const constant = SGroupElement.fromHex(
          "02d6b2141c21e4f337e9b065a031a6269fb5a49253094fc6243d38662eb765db00"
        ).intoConstant();

        expect(constant).toBeDefined();
      });
      it("it should be creatable from byte array", () => {
        const bytes = Uint8Array.from(
          Buffer.from(
            "02d6b2141c21e4f337e9b065a031a6269fb5a49253094fc6243d38662eb765db00",
            "hex"
          )
        );
        const constant = SGroupElement.fromBytes(bytes).intoConstant();

        expect(constant).toBeDefined();
      });
    });
    describe("ErgoBox support", () => {
      it("should create constant from ErgoBox", () => {
        const boxJson = {
          boxId:
            "e56847ed19b3dc6b72828fcfb992fdf7310828cf291221269b7ffc72fd66706e",
          value: 67500000000,
          ergoTree:
            "100204a00b08cd021dde34603426402615658f1d970cfa7c7bd92ac81a8b16eeebff264d59ce4604ea02d192a39a8cc7a70173007301",
          assets: [],
          creationHeight: 284761,
          additionalRegisters: {},
          transactionId:
            "9148408c04c2e38a6402a7950d6157730fa7d49e9ab3b9cadec481d7769918e9",
          index: 1,
        };
        const box = ErgoBox.fromJSON(JSON.stringify(boxJson));
        const constant = new SErgoBox(box).intoConstant();

        expect(constant).toBeDefined();
      });
    });
    // https://github.com/ergoplatform/sigma-rust/blob/166a14ae098ef605ceefccdef53a9a357c088f20/bindings/ergo-lib-wasm/tests/test_constant.js#L146
    describe("EIP-24 R7 monster type", () => {
      it("should produce the same type", () => {
        const constant = SConstant.fromHex(
          "3c0c3c0e0e3c0c3c0e580c3c0e58010301020303040506010301020314160103040506181a"
        );

        expect(constant.typeStr).toBe(
          "STuple([SColl(STuple([SColl(SByte), SColl(SByte)])), STuple([SColl(STuple([SColl(SByte), STuple([SInt, SInt])])), SColl(STuple([SColl(SByte), STuple([SInt, SInt])]))])])"
        );
      });
    });
  });
  describe("Conversion to JS value", () => {
    each([
      [new SUnit(), null],
      [new SBoolean(true), true],
      [new SBoolean(false), false],
      [new SByte(4), 4],
      [new SShort(5), 5],
      [new SInt(11), 11],
      [new SLong(441222311n), 441222311n],
      [new SBigInt(4151222225125102098211n), 4151222225125102098211n],
      [SSigmaProp.fromBool(true), true],
      [SSigmaProp.fromBool(false), false],
    ]).it(
      "should convert simple literal values to the correct JS value",
      (literal, value) => {
        expect(literal.value).toEqual(value);
      }
    );
    it("should convert collection of SInt to array of int", () => {
      const { value } = new SColl([
        new SInt(4),
        new SInt(1),
        new SInt(2),
        new SInt(5),
        new SInt(0),
        new SInt(2),
        new SInt(8),
      ]);
      const expected = [4, 1, 2, 5, 0, 2, 8];

      expect(value).toEqual(expected);
    });
    it("should handle nested collections", () => {
      const { value } = new SColl([
        new SColl([new SBoolean(true), new SBoolean(true)]),
        new SColl([new SBoolean(false), new SBoolean(true)]),
      ]);
      const expected = [
        [true, true],
        [false, true],
      ];

      expect(value).toEqual(expected);
    });
  });
  describe("SColl", () => {
    it("should handle empty collection", () => {
      const nonEmptyColl = new SColl([new SByte(4)]);
      const emptyColl = SColl.emptyOfType(SType.byte());
      const coll = new SColl([nonEmptyColl, emptyColl]).intoConstant();

      expect(coll.typeStr).toBe("SColl(SColl(SByte))");
    });
  });
});
