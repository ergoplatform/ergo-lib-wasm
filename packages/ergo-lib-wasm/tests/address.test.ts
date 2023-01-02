import each from "jest-each";
import {
  Address,
  AddressTypePrefix,
  ErgoTree,
  NetworkAddress,
  NetworkPrefix,
} from "../";

describe("Address", () => {
  describe("fromErgoTree", () => {
    it("should create an address from a P2PK ergo tree", () => {
      const tree = ErgoTree.fromHex(
        "0008cd0327e65711a59378c59359c3e1d0f7abe906479eccb76094e50fe79d743ccc15e6"
      );
      const address = Address.fromErgoTree(tree);

      expect(address).toBeDefined();
    });
    it("should create an address from a P2S ergo tree", () => {
      const tree = ErgoTree.fromHex(
        "100204a00b08cd021dde34603426402615658f1d970cfa7c7bd92ac81a8b16eeebff264d59ce4604ea02d192a39a8cc7a70173007301"
      );
      const address = Address.fromErgoTree(tree);

      expect(address).toBeDefined();
    });
  });
  describe("addressType", () => {
    each(
      [
        "0008cd0327e65711a59378c59359c3e1d0f7abe906479eccb76094e50fe79d743ccc15e6",
        AddressTypePrefix.P2Pk,
      ],
      [
        "100204a00b08cd021dde34603426402615658f1d970cfa7c7bd92ac81a8b16eeebff264d59ce4604ea02d192a39a8cc7a70173007301",
        AddressTypePrefix.Pay2S,
      ]
    ).it(
      "should return the expected address type prefix for the supplied ergo tree",
      (treeHex: string, addressType: AddressTypePrefix) => {
        const tree = ErgoTree.fromHex(treeHex);
        const address = Address.fromErgoTree(tree);

        expect(address.addressType).toBe(addressType);
      }
    );
  });
});

describe("NetworkAddress", () => {
  describe("network()", () => {
    it("should return the correct network prefix", () => {
      const networkAddress = NetworkAddress.fromBase58(
        "3WxrXTcFe672QcgUgtLmjCkZkg98CPMUeojTZbi7vhm8YqN3kzLY"
      );

      expect(networkAddress.networkPrefix).toBe(NetworkPrefix.Testnet);
    });
  });
});
