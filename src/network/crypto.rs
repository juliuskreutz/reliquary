use std::collections::HashMap;

use rand_mt::Mt64;
use tracing::{info, instrument, trace};

use crate::network::bytes_as_hex;

#[instrument(skip_all)]
pub fn decrypt_command(key: &[u8], encrypted: &mut [u8]) {
    trace!(data = bytes_as_hex(encrypted), "before decryption");

    for i in 0..encrypted.len() {
        encrypted[i] ^= key[i % key.len()];
    }

    trace!(data = bytes_as_hex(encrypted), "after decryption");
}

pub fn lookup_initial_key(initial_keys: &HashMap<u16, Vec<u8>>, bytes: &[u8]) -> Vec<u8> {
    let version = u16::from_be_bytes(bytes[..2].try_into().unwrap()) ^ 0x4567;

    // attempt to fetch from user provided initial keys, otherwise use our own baked-in ones
    let key = initial_keys
        .get(&version)
        .map(|k| k.to_vec())
        .unwrap()
        .to_vec();

    info!(version, "found initial decryption key");

    key
}

pub fn new_key_from_seed(seed: u64) -> Vec<u8> {
    // mersenne twister generator
    let mut first = Mt64::new(seed);
    let mut gen = Mt64::new(first.next_u64());
    gen.next_u64();

    let mut key = Vec::with_capacity(512);
    for _ in 0..512 {
        for b in gen.next_u64().to_be_bytes() {
            key.push(b);
        }
    }
    key
}
