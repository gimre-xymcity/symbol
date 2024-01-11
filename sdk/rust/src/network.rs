use chrono::{DateTime, Utc};
use digest::Digest;
use ripemd::Ripemd160;

use crate::byte_array::ByteArray;
use crate::crypto_types::hash256::Hash256;
use crate::crypto_types::public_key::PublicKey;
use crate::network_timestamp_datetime_converter::NetworkTimestampDatetimeConverter;

fn ripemd160(buffer: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(buffer);
    hasher.finalize().to_vec()
}

pub trait NetworkTimestamp {
    fn new(value: i64) -> Self;

    /// Returns `true` if this is epochal timestamp.
    fn is_epochal(&self) -> bool;

    fn timestamp(&self) -> i64;

    /// Adds a specified number of seconds to timestamp.
    fn add_seconds(&self, count: i64) -> Self;

    /// Adds a specified number of minutes to timestamp.
    fn add_minutes(&self, count: i64) -> Self
    where
        Self: Sized,
    {
        self.add_seconds(60 * count)
    }

    /// Adds a specified number of hours to timestamp.
    fn add_hours(&self, count: i64) -> Self
    where
        Self: Sized,
    {
        self.add_minutes(60 * count)
    }
}

pub trait Network {
    type ADDRESS: ByteArray;
    type TIMESTAMP: NetworkTimestamp;

    fn identifier(&self) -> u8;

    fn address_hasher(&self) -> Box<dyn digest::DynDigest>;

    fn create_address(
        &self,
        network_id: u8,
        address_mid: Vec<u8>,
        checksum: &[u8],
    ) -> Self::ADDRESS;

    fn datetime_converter(&self) -> NetworkTimestampDatetimeConverter;

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

    /// Converts a network timestamp to a datetime.
    fn to_datetime(&self, network_timestamp: Self::TIMESTAMP) -> DateTime<Utc> {
        let converter = self.datetime_converter();
        converter.to_datetime(network_timestamp.timestamp())
    }

    /// Converts a datetime to a network timestamp."""
    fn from_datetime(&self, reference_datetime: DateTime<Utc>) -> Self::TIMESTAMP {
        let converter = self.datetime_converter();
        let result = converter.to_difference(reference_datetime);
        Self::TIMESTAMP::new(result)
    }
}
