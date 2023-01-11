use chrono::{DateTime, NaiveDate, Utc};
use digest::Digest;
use lazy_static::lazy_static;
use sha3::Sha3_256;

use self::address::Address;
use crate::byte_array::ByteArray;
use crate::crypto_types::hash256::Hash256;
use crate::network::Network as BasicNetwork;

pub mod address;
pub mod network_timestamp;
pub mod network_timestamp_datetime_converter;

#[derive(Clone, Debug, derive_more::Constructor)]
pub struct Network {
    name: &'static str,
    identifier: u8,
    epoch_time: DateTime<Utc>,
    generation_hash_seed: Hash256,
}

impl BasicNetwork for Network {
    type ADDRESS = Address;

    fn identifier(&self) -> u8 {
        self.identifier
    }

    fn address_hasher(&self) -> Box<dyn digest::DynDigest> {
        Box::new(Sha3_256::new())
    }

    fn create_address(
        &self,
        network_id: u8,
        address_mid: Vec<u8>,
        checksum: &[u8],
    ) -> Self::ADDRESS {
        let mut buffer: [u8; Address::SIZE] = [0; Address::SIZE];
        buffer[0] = network_id;
        buffer[1..21].copy_from_slice(&address_mid[0..20]);
        buffer[21..].copy_from_slice(&checksum[0..3]);
        Address::from(&buffer[..])
    }
}

lazy_static! {
    pub static ref MAINNET: Network = {
        Network::new(
            "mainnet",
            0x68,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 3, 16)
                    .unwrap()
                    .and_hms_opt(0, 6, 25)
                    .unwrap(),
                Utc,
            ),
            Hash256::from("57F7DA205008026C776CB6AED843393F04CD458E0AA2D9F1D5F31A402072B2D6"),
        )
    };
    pub static ref TESTNET: Network = {
        Network::new(
            "testnet",
            0x98,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 11, 25)
                    .unwrap()
                    .and_hms_opt(14, 0, 47)
                    .unwrap(),
                Utc,
            ),
            Hash256::from("49D6E1CE276A85B70EAFE52349AACCA389302E7A9754BCF1221E79494FC665A4"),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_types::public_key::PublicKey;
    use crate::test::network_tests::network_tests;

    network_tests!(
        *MAINNET,
        PublicKey::from("C5FB65CB902623D93DF2E682FFB13F99D50FAC24D5FF2A42F68C7CA1772FE8A0"),
        Address::from("NBLYH55IHPS5QCCMNWR3GZWKV6WMCKPTNKZIBEY")
    );
}
