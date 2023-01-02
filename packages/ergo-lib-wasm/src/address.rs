//! Address types

use ergo_lib::ergo_chain_types::EcPoint;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use ergo_lib::ergotree_ir::sigma_protocol::sigma_boolean::ProveDlog;
use wasm_bindgen::prelude::*;

extern crate derive_more;
use derive_more::{From, Into};

use crate::{ergo_tree::ErgoTree, prelude::MapJsValueErrorResult};

/// Network type
#[wasm_bindgen]
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NetworkPrefix {
    /// Mainnet
    Mainnet = 0,
    /// Testnet
    Testnet = 16,
}

impl From<NetworkPrefix> for ergo_lib::ergotree_ir::chain::address::NetworkPrefix {
    fn from(v: NetworkPrefix) -> Self {
        use ergo_lib::ergotree_ir::chain::address::NetworkPrefix::*;
        match v {
            NetworkPrefix::Mainnet => Mainnet,
            NetworkPrefix::Testnet => Testnet,
        }
    }
}

impl From<ergo_lib::ergotree_ir::chain::address::NetworkPrefix> for NetworkPrefix {
    fn from(v: ergo_lib::ergotree_ir::chain::address::NetworkPrefix) -> Self {
        use NetworkPrefix::*;
        match v {
            ergo_lib::ergotree_ir::chain::address::NetworkPrefix::Mainnet => Mainnet,
            ergo_lib::ergotree_ir::chain::address::NetworkPrefix::Testnet => Testnet,
        }
    }
}

/// Address types
#[wasm_bindgen]
#[repr(u8)]
pub enum AddressTypePrefix {
    /// 0x01 - Pay-to-PublicKey(P2PK) address
    P2Pk = 1,
    /// 0x02 - Pay-to-Script-Hash(P2SH)
    Pay2Sh = 2,
    /// 0x03 - Pay-to-Script(P2S)
    Pay2S = 3,
}

impl From<AddressTypePrefix> for ergo_lib::ergotree_ir::chain::address::AddressTypePrefix {
    fn from(v: AddressTypePrefix) -> Self {
        use ergo_lib::ergotree_ir::chain::address::AddressTypePrefix::*;
        match v {
            AddressTypePrefix::P2Pk => P2Pk,
            AddressTypePrefix::Pay2Sh => Pay2Sh,
            AddressTypePrefix::Pay2S => Pay2S,
        }
    }
}

impl From<ergo_lib::ergotree_ir::chain::address::AddressTypePrefix> for AddressTypePrefix {
    fn from(v: ergo_lib::ergotree_ir::chain::address::AddressTypePrefix) -> Self {
        use AddressTypePrefix::*;
        match v {
            ergo_lib::ergotree_ir::chain::address::AddressTypePrefix::P2Pk => P2Pk,
            ergo_lib::ergotree_ir::chain::address::AddressTypePrefix::Pay2Sh => Pay2Sh,
            ergo_lib::ergotree_ir::chain::address::AddressTypePrefix::Pay2S => Pay2S,
        }
    }
}

/**
 * An address is a short string corresponding to some script used to protect a box. Unlike (string-encoded) binary
 * representation of a script, an address has some useful characteristics:
 *
 * - Integrity of an address could be checked., as it is incorporating a checksum.
 * - A prefix of address is showing network and an address type.
 * - An address is using an encoding (namely, Base58) which is avoiding similarly l0Oking characters, friendly to
 * double-clicking and line-breaking in emails.
 *
 *
 *
 * An address is encoding network type, address type, checksum, and enough information to watch for a particular scripts.
 *
 * Possible network types are:
 * Mainnet - 0x00
 * Testnet - 0x10
 *
 * For an address type, we form content bytes as follows:
 *
 * P2PK - serialized (compressed) public key
 * P2SH - first 192 bits of the Blake2b256 hash of serialized script bytes
 * P2S  - serialized script
 *
 * Address examples for testnet:
 *
 * 3   - P2PK (3WvsT2Gm4EpsM9Pg18PdY6XyhNNMqXDsvJTbbf6ihLvAmSb7u5RN)
 * ?   - P2SH (rbcrmKEYduUvADj9Ts3dSVSG27h54pgrq5fPuwB)
 * ?   - P2S (Ms7smJwLGbUAjuWQ)
 *
 * for mainnet:
 *
 * 9  - P2PK (9fRAWhdxEsTcdb8PhGNrZfwqa65zfkuYHAMmkQLcic1gdLSV5vA)
 * ?  - P2SH (8UApt8czfFVuTgQmMwtsRBZ4nfWquNiSwCWUjMg)
 * ?  - P2S (4MQyML64GnzMxZgm, BxKBaHkvrTvLZrDcZjcsxsF7aSsrN73ijeFZXtbj4CXZHHcvBtqSxQ)
 *
 *
 * Prefix byte = network type + address type
 *
 * checksum = blake2b256(prefix byte ++ content bytes)
 *
 * address = prefix byte ++ content bytes ++ checksum
 *
 */
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct Address(pub(crate) ergo_lib::ergotree_ir::chain::address::Address);

#[wasm_bindgen]
impl Address {
    /// Create an address from a public key
    #[wasm_bindgen(js_name = fromPubKey)]
    pub fn from_public_key(bytes: &[u8]) -> Result<Address, JsValue> {
        EcPoint::sigma_parse_bytes(bytes)
            .map(|point| {
                ergo_lib::ergotree_ir::chain::address::Address::P2Pk(ProveDlog::new(point))
            })
            .map(Address)
            .map_err_js_value()
    }

    /// Decode (base58) address from string without checking the network prefix
    #[wasm_bindgen(js_name = fromBase58)]
    pub fn from_base58(s: &str) -> Result<Address, JsValue> {
        ergo_lib::ergotree_ir::chain::address::AddressEncoder::unchecked_parse_address_from_str(s)
            .map(Address)
            .map_err_js_value()
    }

    /// Decode (base58) address from string without checking the network prefix
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Address, JsValue> {
        ergo_lib::ergotree_ir::chain::address::AddressEncoder::unchecked_parse_address_from_bytes(
            bytes,
        )
        .map(Address)
        .map_err_js_value()
    }

    /// Re-create the address from ErgoTree that was built from the address
    ///
    /// At some point in the past a user entered an address from which the ErgoTree was built.
    /// Re-create the address from this ErgoTree.
    /// `tree` - ErgoTree that was created from an Address
    #[wasm_bindgen(js_name = fromErgoTree)]
    pub fn from_ergo_tree(tree: &ErgoTree) -> Result<Address, JsValue> {
        ergo_lib::ergotree_ir::chain::address::Address::recreate_from_ergo_tree(
            &tree.clone().into(),
        )
        .map(Address)
        .map_err_js_value()
    }

    /// Creates an ErgoTree script from the address
    #[wasm_bindgen(js_name = toErgoTree)]
    pub fn to_ergo_tree(&self) -> Result<ErgoTree, JsValue> {
        self.0
            .script()
            .map(|script| script.into())
            .map_err_js_value()
    }

    /// Get the type of the address
    #[wasm_bindgen(getter, js_name = addressType)]
    pub fn address_type_prefix(&self) -> AddressTypePrefix {
        self.0.address_type_prefix().into()
    }

    /// Returns underlying value for each address type
    /// (serialized EcPoint for P2PK, stored bytes for P2SH and P2S)
    #[wasm_bindgen(getter, js_name = contentBytes)]
    pub fn content_bytes(&self) -> Vec<u8> {
        self.0.content_bytes()
    }
}

/// Combination of an Address with a network
/// These two combined together form a base58 encoding
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkAddress(ergo_lib::ergotree_ir::chain::address::NetworkAddress);

#[wasm_bindgen]
impl NetworkAddress {
    /// create a new NetworkAddress(address + network prefix) for a given network type
    #[wasm_bindgen(constructor)]
    pub fn new(network: NetworkPrefix, address: &Address) -> NetworkAddress {
        NetworkAddress(ergo_lib::ergotree_ir::chain::address::NetworkAddress::new(
            network.into(),
            &address.clone().into(),
        ))
    }

    /// Decode (base58) a NetworkAddress (address + network prefix) from string
    #[wasm_bindgen(js_name = fromBase58)]
    pub fn from_base58(s: &str) -> Result<NetworkAddress, JsValue> {
        ergo_lib::ergotree_ir::chain::address::AddressEncoder::unchecked_parse_network_address_from_str(s)
            .map(NetworkAddress)
            .map_err_js_value()
    }

    /// Encode (base58) address
    #[wasm_bindgen(js_name = toBase58)]
    pub fn to_base58(&self) -> String {
        self.0.to_base58()
    }

    /// Decode from a serialized address
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(data: Vec<u8>) -> Result<NetworkAddress, JsValue> {
        ergo_lib::ergotree_ir::chain::address::AddressEncoder::unchecked_parse_network_address_from_bytes(
            &data,
        )
        .map(NetworkAddress)
        .map_err_js_value()
    }

    /// Encode address as serialized bytes
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        ergo_lib::ergotree_ir::chain::address::AddressEncoder::encode_address_as_bytes(
            self.network().into(),
            &self.address().into(),
        )
    }

    /// Network for the address
    #[wasm_bindgen(getter, js_name = networkPrefix)]
    pub fn network(&self) -> NetworkPrefix {
        self.0.network().into()
    }

    /// Get address without network information
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> Address {
        self.0.address().into()
    }
}
