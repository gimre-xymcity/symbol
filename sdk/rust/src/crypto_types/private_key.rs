use crate::crypto_types::secure_byte_array::SecureByteArray;
use rand::RngCore;
use zeroize::Zeroize;

crate::secure_byte_array!(
    #[derive(Clone)]
    struct PrivateKey; 32; no_str_impl
);

impl PrivateKey {
    pub fn random() -> Self {
        let mut private_key = [0_u8; PrivateKey::SIZE];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut private_key);
        PrivateKey::from(&mut private_key[..])
    }
}

// note: this deliberately does not implement Zeroed trait

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_types::secure_byte_array::*;
    use crate::test_utils;

    #[test]
    fn can_create_with_correct_number_of_bytes() {
        can_create_secure_byte_array_with_correct_number_of_bytes::<PrivateKey>(32);
    }

    #[test]
    #[should_panic]
    fn can_create_with_smaller_number_of_bytes() {
        cannot_create_secure_byte_array_with_smaller_number_of_bytes::<PrivateKey>();
    }

    #[test]
    #[should_panic]
    fn can_create_with_larger_number_of_bytes() {
        cannot_create_secure_byte_array_with_larger_number_of_bytes::<PrivateKey>();
    }

    #[test]
    fn can_create_from_existing() {
        // Arrange:
        let mut raw_bytes = test_utils::rand_bytes(PrivateKey::SIZE);
        let raw_bytes_clone = raw_bytes.clone();

        // Act:
        let instance = PrivateKey::from(&mut raw_bytes[..]);
        let instance_clone = instance.clone();

        // Assert:
        assert_eq!(raw_bytes_clone, instance.as_bytes());
        assert_eq!(raw_bytes_clone, instance_clone.as_bytes());
    }

    #[test]
    fn can_zeroize() {
        can_zeroize_secure_byte_array::<PrivateKey>();
    }

    #[test]
    fn can_create_random_private_key() {
        // Act:
        let private_key_1 = PrivateKey::random();
        let private_key_2 = PrivateKey::random();

        // Assert:
        assert_ne!(private_key_1, private_key_2);
    }

    #[test]
    fn can_serialize_with_serde() {
        let mut data = (0..0xFF).collect::<Vec<u8>>();
        let hash = PrivateKey::from(&mut data[138..138 + 32]);
        let serialized_hash = serde_json::to_string(&hash).unwrap();

        assert_eq!(
            serialized_hash,
            r#"{"bytes":[138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169]}"#
        );
    }

    #[test]
    fn can_deserialize_with_serde() {
        let jsonstr = r#"{"bytes": [141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,
                        157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172]}"#;
        let deserialized: PrivateKey = serde_json::from_str(jsonstr).unwrap();

        assert_eq!(
            deserialized.as_bytes(),
            &[
                141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
                157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172
            ]
        );
    }
}
