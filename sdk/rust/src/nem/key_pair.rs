use crate::crypto_types::ed25519_helpers::derive_public_key;
use crate::crypto_types::hasher512::HashMode;
use crate::crypto_types::key_pair::KeyPair;
use crate::crypto_types::private_key::PrivateKey;
use crate::crypto_types::public_key::PublicKey;
use crate::crypto_types::secure_byte_array::SecureByteArray;

struct NemKeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeyPair for NemKeyPair {
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;

    fn new(mut private_key: PrivateKey) -> Self {
        // TODO: not sure if this should be done here or not, this is how we handle it in py sdk
        private_key.as_bytes_mut().reverse();
        let public_key = derive_public_key(HashMode::Keccak, &private_key);
        Self {
            private_key,
            public_key,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::byte_array::ByteArray;
    use data_encoding::HEXUPPER;

    #[test]
    fn can_create_key_pair_from_private_key() {
        // arrange:
        let mut private_key_data = HEXUPPER
            .decode(b"ED4C70D78104EB11BCD73EBDC512FEBC8FBCEB36A370C957FF7E266230BB5D57")
            .unwrap();
        let deterministic_private_key = PrivateKey::from(&mut private_key_data[..]);

        // act:
        let kp = NemKeyPair::new(deterministic_private_key);

        // assert:
        let expected_public_key = HEXUPPER
            .decode(b"D6C3845431236C5A5A907A9E45BD60DA0E12EFD350B970E7F58E3499E2E7A2F0")
            .unwrap();
        assert_eq!(expected_public_key, kp.public_key.as_bytes());
        assert_eq!(private_key_data, vec![0_u8; 32]);
    }
}
