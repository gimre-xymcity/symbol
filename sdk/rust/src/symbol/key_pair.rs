use crate::crypto_types::ed25519_helpers::derive_public_key;
use crate::crypto_types::hasher512::HashMode;
use crate::crypto_types::key_pair::KeyPair;
use crate::crypto_types::private_key::PrivateKey;
use crate::crypto_types::public_key::PublicKey;

struct SymbolKeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeyPair for SymbolKeyPair {
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;

    fn new(private_key: PrivateKey) -> Self {
        let public_key = derive_public_key(HashMode::Sha2_512, &private_key);
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
            .decode(b"E88283CE35FE74C89FFCB2D8BFA0A2CF6108BDC0D07606DEE34D161C30AC2F1E")
            .unwrap();
        let deterministic_private_key = PrivateKey::from(&mut private_key_data[..]);

        // act:
        let kp = SymbolKeyPair::new(deterministic_private_key);

        // assert:
        let expected_public_key = HEXUPPER
            .decode(b"E29C5934F44482E7A9F50725C8681DE6CA63F49E5562DB7E5BC9EABA31356BAD")
            .unwrap();
        assert_eq!(expected_public_key, kp.public_key.as_bytes());
        assert_eq!(private_key_data, vec![0_u8; 32]);
    }
}
