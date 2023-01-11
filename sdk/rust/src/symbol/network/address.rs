use std::fmt;

use crate::byte_array::ByteArray;
use data_encoding::BASE32_NOPAD;

crate::byte_array!(
	#[derive(Clone)]
	struct Address; 24; no_str_impl);

impl Address {
    fn zero() -> Self {
        Self {
            bytes: [0; Self::SIZE],
        }
    }
}

impl From<&str> for Address {
    fn from(address_str: &str) -> Self {
        let mut address = Self::zero();
        BASE32_NOPAD
            .decode_mut(address_str.as_bytes(), address.as_bytes_mut())
            .unwrap();
        address
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(BASE32_NOPAD.encode(&self.bytes).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byte_array::*;
    use data_encoding::HEXUPPER;

    #[test]
    fn can_create_with_correct_number_of_bytes() {
        can_create_byte_array_with_correct_number_of_bytes::<Address>(24);
    }

    #[test]
    #[should_panic]
    fn cannot_create_with_smaller_number_of_bytes() {
        cannot_create_byte_array_with_smaller_number_of_bytes::<Address>();
    }

    #[test]
    #[should_panic]
    fn cannot_create_with_larger_number_of_bytes() {
        cannot_create_byte_array_with_larger_number_of_bytes::<Address>();
    }

    #[test]
    fn can_create_from_string() {
        // test can create from encoded address
        // Arrange:
        let encoded_address = "TBLYH55IHPS5QCCMNWR3GZWKV6WMCKPTNI7KSDA";
        let decoded = HEXUPPER
            .decode(b"985783F7A83BE5D8084C6DA3B366CAAFACC129F36A3EA90C")
            .unwrap();

        // Act:
        let address = Address::from(encoded_address);

        // Assert:
        assert_eq!(decoded, address.as_bytes());
        assert_eq!(encoded_address, format!("{}", address))
    }

    #[test]
    fn can_create_from_existing() {
        // test can create from address
        // Arrange:
        let encoded_address = "TBLYH55IHPS5QCCMNWR3GZWKV6WMCKPTNI7KSDA";
        let decoded = HEXUPPER
            .decode(b"985783F7A83BE5D8084C6DA3B366CAAFACC129F36A3EA90C")
            .unwrap();
        let original_address = Address::from(encoded_address);

        // Act:
        let address = original_address.clone();

        // Assert:
        assert_eq!(decoded, original_address.as_bytes());
        assert_eq!(decoded, address.as_bytes());
        assert_eq!(encoded_address, format!("{}", address))
    }

    #[test]
    fn can_create_from_bytes() {
        // test can create from decoded address
        // Arrange:
        let encoded_address = "TBLYH55IHPS5QCCMNWR3GZWKV6WMCKPTNI7KSDA";
        let decoded = HEXUPPER
            .decode(b"985783F7A83BE5D8084C6DA3B366CAAFACC129F36A3EA90C")
            .unwrap();

        // Act:
        let address = Address::from(&decoded[..]);

        // Assert:
        assert_eq!(address.as_bytes(), decoded);
        assert_eq!(encoded_address, format!("{}", address))
    }
}
