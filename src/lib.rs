//! [`util.Compress`](https://wiki.facepunch.com/gmod/util.Compress) and [`util.Decompress`](https://wiki.facepunch.com/gmod/util.Decompress) but in Rust!

mod bindings;
use bindings::*;

use std::{convert::TryInto, sync::Once};

static mut NUM_THREADS: i32 = 0;
static NUM_THREADS_SYNC: Once = Once::new();
fn num_cpus() -> i32 {
	unsafe {
		NUM_THREADS_SYNC.call_once(|| {
			NUM_THREADS = num_cpus::get() as i32;
		});
		NUM_THREADS
	}
}

/// LZMA Error code
pub type SZ = size_t;

/// Success
pub const SZ_OK: SZ = 0;
/// Data error
pub const SZ_ERROR_DATA: SZ = 1;
/// Memory allocation error
pub const SZ_ERROR_MEM: SZ = 2;
/// Unsupported properties
pub const SZ_ERROR_UNSUPPORTED: SZ = 4;
/// It needs more bytes in input buffer
pub const SZ_ERROR_INPUT_EOF: SZ = 6;
/// Output buffer overflow
pub const SZ_ERROR_OUTPUT_EOF: SZ = 7;
/// Errors in multithreading functions
pub const SZ_ERROR_THREAD: SZ = 12;

const LZMA_PROPS_SIZE: usize = 5;

/// 🔮 [`util.Compress`](https://wiki.facepunch.com/gmod/util.Compress)
///
/// Compress a slice of bytes.
///
/// ## Example
/// ```rust
/// let original = b"Hello, world!";
/// let compressed = gmod_lzma::compress(original, 9).unwrap();
/// let decompressed = gmod_lzma::decompress(&compressed).unwrap();
/// assert_eq!(decompressed, original);
/// ```
///
/// ## Error codes
/// |||
/// |---|---|
/// | `SZ_OK` | Success |
/// | `SZ_ERROR_MEM` | Memory allocation error |
/// | `SZ_ERROR_OUTPUT_EOF` | Output buffer overflow |
/// | `SZ_ERROR_THREAD` | Errors in multithreading functions |
pub fn compress(data: &[u8], level: i32) -> Result<Vec<u8>, SZ> {
	unsafe {
		let input_len = data.len();

		let mut dest_size = (input_len + input_len / 3 + 128) as size_t;

		let mut output = vec![0u8; dest_size as usize];

		let mut props_size = LZMA_PROPS_SIZE as size_t;
		let res = LzmaCompress(
			output.as_mut_ptr().add(8 + LZMA_PROPS_SIZE),
			&mut dest_size as *mut _,
			data.as_ptr(),
			input_len as size_t,
			output.as_mut_ptr(),
			&mut props_size as *mut _,
			level,
			1 << 16,
			3,
			0,
			2,
			32,
			num_cpus(),
		) as SZ;

		if props_size != LZMA_PROPS_SIZE as size_t {
			return Err(SZ_ERROR_UNSUPPORTED);
		}

		if res != SZ_OK {
			return Err(res);
		}

		let input_len = input_len as size_t;
		for (i, byte) in input_len.to_le_bytes().iter().enumerate() {
			output[i + LZMA_PROPS_SIZE] = *byte;
		}

		output.truncate(dest_size as usize + LZMA_PROPS_SIZE + 8);
		output.shrink_to_fit();

		Ok(output)
	}
}

/// 🔮 [`util.Decompress`](https://wiki.facepunch.com/gmod/util.Decompress)
///
/// Decompress a compressed slice of bytes.
///
/// ## Example
/// ```rust
/// let original = b"Hello, world!";
/// let compressed = gmod_lzma::compress(original, 9).unwrap();
/// let decompressed = gmod_lzma::decompress(&compressed).unwrap();
/// assert_eq!(decompressed, original);
/// ```
///
/// ## Error codes
/// |||
/// |---|---|
/// | `SZ_OK` | Success |
/// | `SZ_ERROR_DATA` | Data error |
/// | `SZ_ERROR_MEM` | Memory allocation arror |
/// | `SZ_ERROR_UNSUPPORTED` | Unsupported properties |
/// | `SZ_ERROR_INPUT_EOF` | It needs more bytes in input buffer |
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, SZ> {
	unsafe {
		let dest_len = u64::from_le_bytes(
			data.get(LZMA_PROPS_SIZE..LZMA_PROPS_SIZE + 8)
				.ok_or(SZ_ERROR_INPUT_EOF)?
				.try_into()
				.unwrap(),
		);
		let mut written = dest_len as size_t;
		let mut output = vec![0u8; dest_len as usize];
		let mut src_len = data.len() as size_t - LZMA_PROPS_SIZE as size_t - 8;

		let res = LzmaUncompress(
			output.as_mut_ptr(),
			&mut written as *mut _,
			data.as_ptr().add(LZMA_PROPS_SIZE + 8),
			&mut src_len as *mut _,
			data.as_ptr(),
			LZMA_PROPS_SIZE as size_t,
		) as SZ;

		if res != SZ_OK {
			return Err(res);
		}

		Ok(output)
	}
}

#[test]
fn test_compress() {
	let data = b"Hello world";
	let bytes = compress(data, 9).unwrap();
	assert_eq!(
		bytes,
		&[
			0x5D, 0x00, 0x00, 0x01, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x24, 0x19, 0x49, 0x98, 0x6F, 0x10, 0x19, 0xC6, 0xD7, 0x31, 0xEB, 0x36, 0x11, 0xEF,
			0x00
		]
	);
}

#[test]
fn test_decompress() {
	let data = b"Hello world";
	let bytes = compress(data, 9).unwrap();
	assert_eq!(
		bytes,
		&[
			0x5D, 0x00, 0x00, 0x01, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x24, 0x19, 0x49, 0x98, 0x6F, 0x10, 0x19, 0xC6, 0xD7, 0x31, 0xEB, 0x36, 0x11, 0xEF,
			0x00
		]
	);
	let bytes = decompress(&bytes).unwrap();
	assert_eq!(bytes, data);
}

#[test]
fn test_decompress_error() {
	assert!(decompress(b"Error!!!").is_err());
}
