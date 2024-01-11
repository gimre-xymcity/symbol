use crate::byte_array::ByteArray;
use crate::crypto_types::secure_byte_array::SecureByteArray;

pub trait KeyPair {
    type PrivateKey: SecureByteArray;
    type PublicKey: ByteArray;

    fn new(private_key: Self::PrivateKey) -> Self;
}
