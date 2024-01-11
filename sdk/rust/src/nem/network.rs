use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use digest::Digest;
use lazy_static::lazy_static;
use sha3::Keccak256;

use self::address::Address;
use self::network_timestamp::NetworkTimestamp;
use crate::byte_array::ByteArray;
use crate::network::Network as BasicNetwork;
use crate::network_timestamp_datetime_converter::{NetworkTimestampDatetimeConverter, TimeUnits};

pub mod address;
pub mod network_timestamp;

#[derive(Clone, Debug, derive_more::Constructor)]
pub struct Network {
    name: &'static str,
    identifier: u8,
    epoch_time: DateTime<Utc>,
}

impl BasicNetwork for Network {
    type ADDRESS = Address;
    type TIMESTAMP = NetworkTimestamp;

    fn identifier(&self) -> u8 {
        self.identifier
    }

    fn address_hasher(&self) -> Box<dyn digest::DynDigest> {
        Box::new(Keccak256::new())
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
        buffer[21..].copy_from_slice(&checksum[0..4]);
        Address::from(&buffer[..])
    }

    fn datetime_converter(&self) -> NetworkTimestampDatetimeConverter {
        NetworkTimestampDatetimeConverter {
            epoch: self.epoch_time,
            time_units: TimeUnits::Seconds,
        }
    }
}

lazy_static! {
    static ref MAINNET: Network = {
        Network::new(
            "mainnet",
            0x68,
            Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2015, 3, 29)
                    .unwrap()
                    .and_hms_opt(0, 6, 25)
                    .unwrap(),
            ),
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
        PublicKey::from("D6C3845431236C5A5A907A9E45BD60DA0E12EFD350B970E7F58E3499E2E7A2F0"),
        Address::from("NCFGSLITSWMRROU2GO7FPMIUUDELUPSZUNJABUMH")
    );
}
