use bitcoin::{
    BlockHeader, Address,
    consensus::deserialize,
};
use sha2::{Sha256, Digest};
use hex;

use std::str::FromStr;

pub fn decode_block_header(hex_encoded: String) -> BlockHeader {
    let raw = hex::decode(&hex_encoded).unwrap();
    let block_header = deserialize(&raw).unwrap();
    block_header
}

// TODO(evg): something better ??
pub fn decode_block_header_chain(hex_encoded: String) -> Vec<BlockHeader> {
    let mut vec = Vec::new();
    let raw = hex::decode(&hex_encoded).unwrap();
    for left in (0..raw.len()).step_by(80) {
        let right = left + 80;
        let block_header = deserialize(&raw[left..right]).unwrap();
        vec.push(block_header);
    }
    return vec
}

pub fn decode_address_helper(addr: &str) -> String {
    let addr = Address::from_str(&addr).unwrap();
    let locking_script = addr.script_pubkey();

    let mut hasher = Sha256::new();
    hasher.input(locking_script.as_bytes());
    let mut result = hasher.result();

    result.reverse();
    format!("{:x}", result)
}