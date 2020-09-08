[![Current Crates.io Version](https://img.shields.io/crates/v/hexlit.svg)](https://crates.io/crates/hexlit)
[![docs-rs](https://docs.rs/hexlit/badge.svg)](https://docs.rs/hexlit)
![MSRV 1.46+](https://img.shields.io/badge/rustc-1.46+-blue.svg)

# hexlit
A zero-allocation no_std-compatible zero-cost way to convert hex-strings to byte-arrays at compile time.

To add to your Cargo.toml:
```toml
hexlit = "0.2.0"
```

## Example
```rust
use hexlit::hex;

fn main() {
const DATA: [u8; 4] = hex!("01020304");
assert_eq!(DATA, [1, 2, 3, 4]);
assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
}
```