use std::hash::Hash;

use super::hash512::Hash512;
use super::hasher512::{HashMode, Hasher512};
use super::private_key::PrivateKey;
use super::public_key::PublicKey;
use super::secure_byte_array::SecureByteArray;

use crate::byte_array::{ByteArray, Zeroed};

use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::scalar::Scalar;
use zeroize::{Zeroize, Zeroizing};

/// Length of an ed25519 secret key, in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

/// Length of a private key hash, in bytes.
pub const HASH_LENGTH: usize = 64;

fn hash_private_key<T: SecureByteArray>(hash_mode: HashMode, sk: &T) -> Zeroizing<Hash512> {
    let mut hasher = Hasher512::new(hash_mode);
    hasher.update(sk.as_bytes());

    let mut hash: Hash512 = Hash512::zero();
    hasher.finalize_into(hash.as_bytes_mut());

    Zeroizing::new(hash)
}

fn copy_and_clamp_private_hash(private_hash: &Hash512) -> Zeroizing<Scalar> {
    let mut bits =
        <[u8; SECRET_KEY_LENGTH]>::try_from(&private_hash.as_bytes()[..SECRET_KEY_LENGTH]).unwrap();

    bits[0] &= 248;
    bits[31] &= 127;
    bits[31] |= 64;

    let clamped_scalar = Scalar::from_bytes_mod_order(*&mut bits);
    bits.zeroize();
    Zeroizing::new(clamped_scalar)
}

fn hash_to_scalar(builder: Hasher512) -> Scalar {
    let mut hash = [0u8; HASH_LENGTH];
    builder.finalize_into(&mut hash);
    Scalar::from_bytes_mod_order_wide(&hash)
}

fn generate_nonce(hash_mode: HashMode, private_hash: &[u8; HASH_LENGTH], message: &[u8]) -> Scalar {
    // hash half of priv key hash
    let mut builder = Hasher512::new(hash_mode);
    builder.update(&private_hash[32..64]);
    builder.update(&message);

    hash_to_scalar(builder)
}

pub(crate) fn derive_public_key(hash_mode: HashMode, private_key: &PrivateKey) -> PublicKey {
    let private_hash = hash_private_key(hash_mode, private_key);
    let key = copy_and_clamp_private_hash(&private_hash);
    let point = Zeroizing::new(EdwardsPoint::mul_base(&*key));

    let compressed = point.compress().to_bytes();
    PublicKey::from(&compressed[..])
}
