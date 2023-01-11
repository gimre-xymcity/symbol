#[cfg(test)]
macro_rules! network_tests {
    ($network:expr, $deterministic_public_key:expr, $expected_mainnet_address:expr) => {
        #[test]
        fn test_can_convert_mainnet_public_key_to_address() {
            // Act:
            let address = $network.public_key_to_address(&$deterministic_public_key);

            // Assert:
            assert_eq!($expected_mainnet_address, address);
        }

        #[test]
        fn test_can_validate_valid_address() {
            // Arrange:
            let address = $network.public_key_to_address(&$deterministic_public_key);

            // Act + Assert
            assert!($network.is_valid_address(address));
            //assert!($network.is_valid_address_string(address));
        }

        #[test]
        fn test_cannot_validate_invalid_address_begin() {
            // Arrange:
            let mut address = $network.public_key_to_address(&$deterministic_public_key);
            let address_bytes = address.as_bytes_mut();
            address_bytes[1] ^= 0xFF;

            // Act + Assert
            assert!(!$network.is_valid_address(address));
            //assert!(! $network.is_valid_address_string(address));
        }

        #[test]
        fn test_cannot_validate_invalid_address_end() {
            // Arrange:
            let mut address = $network.public_key_to_address(&$deterministic_public_key);
            let address_bytes = address.as_bytes_mut();
            address_bytes[address_bytes.len() - 1] ^= 0xFF;

            // Act + Assert
            assert!(!$network.is_valid_address(address));
            //assert!(! $network.is_valid_address_string(address));
        }
    };
}

pub(crate) use network_tests;
