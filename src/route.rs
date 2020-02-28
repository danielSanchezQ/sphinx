use crate::constants::{DESTINATION_ADDRESS_LENGTH, IDENTIFIER_LENGTH, NODE_ADDRESS_LENGTH};
use crate::crypto;

// in paper delta
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct DestinationAddressBytes([u8; DESTINATION_ADDRESS_LENGTH]);

impl DestinationAddressBytes {
    pub fn to_base58_string(&self) -> String {
        bs58::encode(&self.0).into_string()
    }

    pub fn from_base58_string(value: String) -> Self {
        let decoded_address = bs58::decode(&value).into_vec().unwrap();
        assert_eq!(decoded_address.len(), DESTINATION_ADDRESS_LENGTH);
        let mut address_bytes = [0; DESTINATION_ADDRESS_LENGTH];
        address_bytes.copy_from_slice(&decoded_address[..]);

        DestinationAddressBytes(address_bytes)
    }

    pub fn from_bytes(b: [u8; DESTINATION_ADDRESS_LENGTH]) -> Self {
        DestinationAddressBytes(b)
    }

    /// View this `DestinationAddressBytes` as an array of bytes.
    pub fn as_bytes(&self) -> &[u8; DESTINATION_ADDRESS_LENGTH] {
        &self.0
    }

    /// Convert this `DestinationAddressBytes` to an array of bytes.
    pub fn to_bytes(&self) -> [u8; DESTINATION_ADDRESS_LENGTH] {
        self.0
    }
}

// in paper nu
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct NodeAddressBytes([u8; NODE_ADDRESS_LENGTH]);

impl NodeAddressBytes {
    pub fn to_base58_string(&self) -> String {
        bs58::encode(&self.0).into_string()
    }

    pub fn from_base58_string(value: String) -> Self {
        let decoded_address = bs58::decode(&value).into_vec().unwrap();
        assert_eq!(decoded_address.len(), NODE_ADDRESS_LENGTH);
        let mut address_bytes = [0; NODE_ADDRESS_LENGTH];
        address_bytes.copy_from_slice(&decoded_address[..]);

        NodeAddressBytes(address_bytes)
    }

    pub fn from_bytes(b: [u8; NODE_ADDRESS_LENGTH]) -> Self {
        NodeAddressBytes(b)
    }

    /// View this `NodeAddressBytes` as an array of bytes.
    pub fn as_bytes(&self) -> &[u8; NODE_ADDRESS_LENGTH] {
        &self.0
    }

    /// Convert this `NodeAddressBytes` to an array of bytes.
    pub fn to_bytes(&self) -> [u8; NODE_ADDRESS_LENGTH] {
        self.0
    }
}

// in paper I
pub type SURBIdentifier = [u8; IDENTIFIER_LENGTH];

#[derive(Debug, PartialEq)]
pub struct Destination {
    // address in theory could be changed to a vec<u8> as it does not need to be strictly DESTINATION_ADDRESS_LENGTH long
    // but cannot be longer than that (assuming longest possible route)
    pub address: DestinationAddressBytes,
    pub identifier: SURBIdentifier,
}

impl Destination {
    pub fn new(address: DestinationAddressBytes, identifier: SURBIdentifier) -> Self {
        Self {
            address,
            identifier,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub address: NodeAddressBytes,
    pub pub_key: crypto::PublicKey,
}

impl Node {
    pub fn new(address: NodeAddressBytes, pub_key: crypto::PublicKey) -> Self {
        Self { address, pub_key }
    }
}

pub fn destination_address_fixture() -> DestinationAddressBytes {
    DestinationAddressBytes([0u8; DESTINATION_ADDRESS_LENGTH])
}

pub fn node_address_fixture() -> NodeAddressBytes {
    NodeAddressBytes([0u8; NODE_ADDRESS_LENGTH])
}

pub fn surb_identifier_fixture() -> SURBIdentifier {
    [0u8; IDENTIFIER_LENGTH]
}

pub fn random_node() -> Node {
    Node {
        address: NodeAddressBytes([2u8; NODE_ADDRESS_LENGTH]),
        pub_key: crypto::generate_random_curve_point(),
    }
}

pub fn destination_fixture() -> Destination {
    Destination {
        address: DestinationAddressBytes([3u8; DESTINATION_ADDRESS_LENGTH]),
        identifier: [4u8; IDENTIFIER_LENGTH],
    }
}

#[cfg(test)]
mod address_encoding {
    use super::*;

    #[test]
    fn it_is_possible_to_encode_and_decode_address() {
        let dummy_address = NodeAddressBytes([42u8; 32]);
        let dummy_address_str = dummy_address.to_base58_string();
        let recovered = NodeAddressBytes::from_base58_string(dummy_address_str);
        assert_eq!(dummy_address, recovered)
    }
}
