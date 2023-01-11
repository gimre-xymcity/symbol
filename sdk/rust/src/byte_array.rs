pub trait ByteArray {
    //const fn len() -> usize;
    const SIZE: usize;

    fn as_bytes(&self) -> &[u8];

    fn as_bytes_mut(&mut self) -> &mut [u8];
}

pub trait Zeroed {
    fn zero() -> Self;
}

#[macro_export]
macro_rules! byte_array {
	(
		$(#[$meta:meta])*
		struct $struct_name:ident; $byte_size:literal; no_str_impl
	) => {
		$(#[$meta])*
        #[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
        pub struct $struct_name {
            bytes: [u8; $byte_size],
        }

        impl $crate::byte_array::ByteArray for $struct_name {
            const SIZE: usize = $byte_size;

            fn as_bytes(&self) -> &[u8] {
                &self.bytes
            }

            fn as_bytes_mut(&mut self) -> &mut [u8] {
                &mut self.bytes
            }
        }

		impl From<&[u8]> for $struct_name {
            fn from(other: &[u8]) -> Self {
                assert_eq!(Self::SIZE, other.len());
                $struct_name {
                    bytes: other[0..Self::SIZE].try_into().expect("invalid size"),
                }
            }
        }
	};
    (
		$(#[$meta:meta])*
		struct $struct_name:ident; $byte_size:literal; str_impl
	) => {
		$crate::byte_array!(
			$(#[$meta])*
			struct $struct_name; $byte_size; no_str_impl);

		impl From<&str> for $struct_name {
			fn from(hex_str: &str) -> Self {
				let mut obj = Self{ bytes: [0; $byte_size] };
				data_encoding::HEXUPPER
					.decode_mut(hex_str.as_bytes(), obj.as_bytes_mut())
					.unwrap();
				obj
			}
		}
    };
}

#[cfg(test)]
use crate::test_utils;

#[cfg(test)]
pub fn can_create_byte_array_with_correct_number_of_bytes<T: for<'a> From<&'a [u8]> + ByteArray>(
    size: usize,
) {
    // Arrange:
    let raw_bytes = test_utils::rand_bytes(size);

    // Act:
    let instance: T = <T>::from(&raw_bytes);

    // Assert:
    assert_eq!(T::SIZE, size);
    assert_eq!(std::mem::size_of::<T>(), size);
    assert_eq!(raw_bytes, instance.as_bytes());
}

#[cfg(test)]
#[allow(unused_must_use)]
pub fn cannot_create_byte_array_with_smaller_number_of_bytes<
    T: for<'a> From<&'a [u8]> + ByteArray,
>() {
    // Arrange:
    let raw_bytes = test_utils::rand_bytes(T::SIZE - 1);

    // Act + Assert:
    <T>::from(&raw_bytes);
}

#[cfg(test)]
#[allow(unused_must_use)]
pub fn cannot_create_byte_array_with_larger_number_of_bytes<
    T: for<'a> From<&'a [u8]> + ByteArray,
>() {
    // Arrange:
    let raw_bytes = test_utils::rand_bytes(T::SIZE + 1);

    // Act + Assert:
    <T>::from(&raw_bytes);
}

#[cfg(test)]
pub fn can_create_zeroed_byte_array<T: ByteArray + Zeroed>(size: usize) {
    // Act:
    let instance: T = <T>::zero();

    // Assert:
    assert_eq!(vec![0; size], instance.as_bytes());
}
