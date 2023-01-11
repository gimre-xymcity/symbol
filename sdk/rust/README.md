# Symbol SDK

`symbol_sdk` is a minimal rust SDK for Symbol and NEM. The architecture and programming paradigm of this SDK are consistent with those for other languages.


## notes specific to rust implementation

### byte_array

Byte arrays are implemented using `byte_array!` macro.

Some byte arrays implement `zero()`. It should be considered if using `Default` trait would be better choice.

### tests

there are two alternative approaches when it comes to tests:
 * slightly complicated, using Traits an generic functions - see address tests (symbol/network/address.rs and nem/network/adddress.rs)
 * using macros - see network tests
