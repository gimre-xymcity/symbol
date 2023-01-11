use digest::Digest;
use ripemd::Ripemd160;

use crate::byte_array::ByteArray;
use crate::crypto_types::hash256::Hash256;
use crate::crypto_types::public_key::PublicKey;

fn ripemd160(buffer: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(buffer);
    hasher.finalize().to_vec()
}

pub trait Network {
    type ADDRESS: ByteArray;

    fn identifier(&self) -> u8;

    fn address_hasher(&self) -> Box<dyn digest::DynDigest>;

    fn create_address(
        &self,
        network_id: u8,
        address_mid: Vec<u8>,
        checksum: &[u8],
    ) -> Self::ADDRESS;

    fn public_key_to_address(&self, public_key: &PublicKey) -> Self::ADDRESS {
        let mut part_one_hash_builder = self.address_hasher();
        part_one_hash_builder.update(public_key.as_bytes());
        let part_one_hash = Hash256::from(&*part_one_hash_builder.finalize());

        let part_two = ripemd160(part_one_hash.as_bytes());

        let network_id = self.identifier();
        let mut part_three_hash_builder = self.address_hasher();
        part_three_hash_builder.update(&[network_id]);
        part_three_hash_builder.update(part_two.as_ref());
        let checksum_hash = part_three_hash_builder.finalize();
        let checksum = &(*checksum_hash);

        self.create_address(network_id, part_two, checksum)
    }

    /// Checks if an address is valid and belongs to this network.
    fn is_valid_address(&self, address: Self::ADDRESS) -> bool {
        let address_bytes = address.as_bytes();
        if address_bytes[0] != self.identifier() {
            return false;
        }

        let mut hash_builder = self.address_hasher();
        hash_builder.update(&address_bytes[0..21]);
        let checksum_hash = hash_builder.finalize();
        address_bytes[21..] == checksum_hash[0..address_bytes.len() - 21]
    }
}
