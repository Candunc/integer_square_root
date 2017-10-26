Integer Square Root
===================

This library aims to provide the square root function for all sized integer types currently supported by Rust. It avoids casting to f64, however at a massive cost - expect non-casted square roots to take ~20 ns longer, as there is no hardware instruction for integer square roots.

Example
-------

```rust
extern crate integer_square_root;
use integer_square_root::SquareRoot;

asserteq!(16u8.sqrt(),4u8);
```