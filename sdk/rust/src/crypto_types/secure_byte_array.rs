//use zeroize::Zeroizing;

pub trait SecureByteArray {
    //const fn len() -> usize;
    const SIZE: usize;

    fn as_bytes(&self) -> &[u8];

    fn as_bytes_mut(&mut self) -> &mut [u8];
}

pub trait Zeroed {
    fn zero() -> Self;
}

#[macro_export]
macro_rules! secure_byte_array {
    (
        $(#[$meta:meta])*
        struct $struct_name:ident; $byte_size:literal; no_str_impl
    ) => {
        $(#[$meta])*
        #[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
        pub struct $struct_name {
            bytes: zeroize::Zeroizing<[u8; $byte_size]>,
        }

        impl $crate::crypto_types::secure_byte_array::SecureByteArray for $struct_name {
            const SIZE: usize = $byte_size;

            fn as_bytes(&self) -> &[u8] {
                &self.bytes.as_ref()
            }

            fn as_bytes_mut(&mut self) -> &mut [u8] {
                self.bytes.as_mut()
            }
        }

        impl Zeroize for $struct_name {
            fn zeroize(&mut self) {
                self.bytes.zeroize()
            }
        }

        impl From<&mut [u8]> for $struct_name {
            fn from(other: &mut [u8]) -> Self {
                assert_eq!(Self::SIZE, other.len());
                let r = $struct_name {
                    bytes: zeroize::Zeroizing::new(other[0..Self::SIZE].try_into().expect("invalid size")),
                };
                other.zeroize();
                r
            }
        }
    };
    // (
    //     $(#[$meta:meta])*
    //     struct $struct_name:ident; $byte_size:literal; str_impl
    // ) => {
    //     $crate::byte_array!(
    //         $(#[$meta])*
    //         struct $struct_name; $byte_size; no_str_impl);

    //     impl From<&str> for $struct_name {
    //         fn from(hex_str: &str) -> Self {
    //             let mut obj = Self{ bytes: [0; $byte_size] };
    //             data_encoding::HEXUPPER
    //                 .decode_mut(hex_str.as_bytes(), obj.as_bytes_mut())
    //                 .unwrap();
    //             obj
    //         }
    //     }
    // };
}

#[cfg(test)]
use crate::test_utils;

#[cfg(test)]
use zeroize::Zeroize;

#[cfg(test)]
pub fn can_create_secure_byte_array_with_correct_number_of_bytes<
    T: for<'a> From<&'a mut [u8]> + SecureByteArray,
>(
    size: usize,
) {
    // Arrange:

    let mut raw_bytes = test_utils::rand_bytes(size);
    let raw_bytes_clone = raw_bytes.clone();

    // Act:
    let instance: T = <T>::from(&mut raw_bytes[..]);

    // Assert:
    assert_eq!(T::SIZE, size);
    assert_eq!(std::mem::size_of::<T>(), size);
    assert_eq!(raw_bytes_clone, instance.as_bytes());
    assert_eq!(&[0_u8; 32], raw_bytes.as_slice());
}

#[cfg(test)]
pub fn can_zeroize_secure_byte_array<T: for<'a> From<&'a mut [u8]> + SecureByteArray + Zeroize>() {
    // Arrange:

    let mut raw_bytes = test_utils::rand_bytes(T::SIZE);
    let raw_bytes_clone = raw_bytes.clone();
    let mut instance: T = <T>::from(&mut raw_bytes[..]);

    // Act:
    instance.zeroize();

    // Assert:
    assert_ne!(raw_bytes_clone, instance.as_bytes());
    assert_eq!(&[0_u8; 32], instance.as_bytes());
}

#[cfg(test)]
#[allow(unused_must_use)]
pub fn cannot_create_secure_byte_array_with_smaller_number_of_bytes<
    T: for<'a> From<&'a mut [u8]> + SecureByteArray,
>() {
    // Arrange:
    let mut raw_bytes = test_utils::rand_bytes(T::SIZE - 1);

    // Act + Assert:
    <T>::from(&mut raw_bytes);
}

#[cfg(test)]
#[allow(unused_must_use)]
pub fn cannot_create_secure_byte_array_with_larger_number_of_bytes<
    T: for<'a> From<&'a mut [u8]> + SecureByteArray,
>() {
    // Arrange:
    let mut raw_bytes = test_utils::rand_bytes(T::SIZE + 1);

    // Act + Assert:
    <T>::from(&mut raw_bytes);
}

#[cfg(test)]
pub fn can_create_zeroed_secure_byte_array<T: SecureByteArray + Zeroed>(size: usize) {
    // Act:
    let instance: T = <T>::zero();

    // Assert:
    assert_eq!(vec![0; size], instance.as_bytes());
}
