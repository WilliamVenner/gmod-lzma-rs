[![crates.io](https://img.shields.io/crates/v/gmod-lzma.svg)](https://crates.io/crates/gmod-lzma)
[![docs.rs](https://docs.rs/gmod-lzma/badge.svg)](https://docs.rs/gmod-lzma/)
[![license](https://img.shields.io/crates/l/gmod-lzma)](https://github.com/WilliamVenner/gmod-lzma-rs/blob/master/LICENSE)

# 🔮 gmod-lzma-rs

[`util.Compress`](https://wiki.facepunch.com/gmod/util.Compress) and [`util.Decompress`](https://wiki.facepunch.com/gmod/util.Decompress) but in Rust!

# Usage

## As a Rust library

Add to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) file:

```toml
[dependencies]
gmod-lzma = "*"
```

## As a binary

You can download a binary from the [releases](https://github.com/WilliamVenner/gmod-lzma-rs/releases) page.

```php
Usage: gmod_lzma (-c | --compress) [-l | --level <level>] <input> <output>
       gmod_lzma (-d | --decompress) <input> <output>
```
