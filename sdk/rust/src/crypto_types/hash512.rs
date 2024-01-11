use crate::byte_array::{ByteArray, Zeroed};
use serde_with::{serde_as, Bytes};
use zeroize::Zeroize;

crate::byte_array!(
    #[derive(Clone)]
    struct Hash512; 64; str_impl
);

impl Zeroed for Hash512 {
    fn zero() -> Self {
        Self {
            bytes: [0; Self::SIZE],
        }
    }
}

impl Zeroize for Hash512 {
    fn zeroize(&mut self) {
        self.bytes.zeroize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byte_array::*;
    use crate::test_utils;

    #[test]
    fn can_create_with_correct_number_of_bytes() {
        can_create_byte_array_with_correct_number_of_bytes::<Hash512>(64);
    }

    #[test]
    #[should_panic]
    fn cannot_create_with_smaller_number_of_bytes() {
        cannot_create_byte_array_with_smaller_number_of_bytes::<Hash512>();
    }

    #[test]
    #[should_panic]
    fn cannot_create_with_larger_number_of_bytes() {
        cannot_create_byte_array_with_larger_number_of_bytes::<Hash512>();
    }

    #[test]
    fn can_zero_init() {
        can_create_zeroed_byte_array::<Hash512>(64);
    }

    #[test]
    fn can_create_from_existing() {
        // Arrange:
        let raw_bytes = test_utils::rand_bytes(Hash512::SIZE);

        // Act:
        let instance = Hash512::from(&raw_bytes[..]);
        let instance_clone = instance.clone();

        // Assert:
        assert_eq!(raw_bytes, instance.as_bytes());
        assert_eq!(raw_bytes, instance_clone.as_bytes());
    }

    #[test]
    fn can_serialize_with_serde() {
        let data = (0..0xFF).collect::<Vec<u8>>();
        let hash = Hash512::from(&data[138..138 + 64]);
        let serialized_hash = serde_json::to_string(&hash).unwrap();

        assert_eq!(
            serialized_hash,
            r#"{"bytes":[138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201]}"#
        );
    }

    #[test]
    fn can_deserialize_with_serde() {
        let jsonstr = r#"{"bytes": [
						141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,
                        157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,
						173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,
						189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204]}"#;
        let deserialized: Hash512 = serde_json::from_str(jsonstr).unwrap();

        assert_eq!(
            deserialized.as_bytes(),
            &[
                141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
                157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
                173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188,
                189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204
            ]
        );
    }
}
