use sha2::{Digest as Sha2Digest, Sha512};
use sha3::Keccak512;

/// Length of a private key hash, in bytes.
pub const HASH_LENGTH: usize = 64;

/// Supported hash modes.
#[derive(Copy, Clone, PartialEq)]
pub enum HashMode {
    /// Keccak hash.
    Keccak,

    /// SHA2 hash.
    Sha2_512,
}

pub struct Hasher512 {
    hash_mode: HashMode,
    _keccak: Keccak512,
    _sha2: Sha512,
}

impl Hasher512 {
    pub fn new(hash_mode: HashMode) -> Hasher512 {
        return Hasher512 {
            hash_mode: hash_mode,
            _keccak: Keccak512::new(),
            _sha2: Sha512::new(),
        };
    }

    pub fn update<T: AsRef<[u8]> + ?Sized>(&mut self, input: &T) {
        match self.hash_mode {
            HashMode::Keccak => self._keccak.update(&input),
            HashMode::Sha2_512 => self._sha2.update(&input),
        }
    }

    pub fn finalize_into(self, out: &mut [u8]) {
        let finalized = match self.hash_mode {
            HashMode::Keccak => self._keccak.finalize(),
            HashMode::Sha2_512 => self._sha2.finalize(),
        };
        out.copy_from_slice(&finalized.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::{byte_array::*, crypto_types::hash256::*, crypto_types::hash512::*};

    use super::*;

    #[test]
    fn can_instantiate_and_calculate_sha2_512_hasher() {
        // Arrange:
        let input =
            Hash256::from("9F2FCC7C90DE090D6B87CD7E9718C1EA6CB21118FC2D5DE9F97E5DB6AC1E9C10");
        let mut output = Hash512::zero();

        // Act:
        let mut hasher = Hasher512::new(HashMode::Sha2_512);
        hasher.update(input.as_bytes());
        hasher.finalize_into(output.as_bytes_mut());

        // Assert:
        let expected = Hash512::from("FF6EBF72E7E9BC05E06A3DDBAC4298B68DCF50374BD74E910977A496F41270931268FABB3774B73EEC64E5D729C75D0887112E2FAD4DFA7DCEB8D1D97A3DFE44");
        assert_eq!(output, expected);
    }

    #[test]
    fn can_instantiate_and_calculate_keccak_512_hasher() {
        // Arrange:
        let input =
            Hash256::from("9F2FCC7C90DE090D6B87CD7E9718C1EA6CB21118FC2D5DE9F97E5DB6AC1E9C10");
        let mut output = Hash512::zero();

        // Act:
        let mut hasher = Hasher512::new(HashMode::Keccak);
        hasher.update(input.as_bytes());
        hasher.finalize_into(output.as_bytes_mut());

        // Assert:
        let expected = Hash512::from("1EAFEDCE7292BA73B80AE6151745F43AC95BFC9F31694D422473ABCA2E69D695CB6544DB65506078CB20DBE0762F84AA6AFD14A60AB597955BE73F3F5C50F7A8");
        assert_eq!(output, expected);
    }
}
