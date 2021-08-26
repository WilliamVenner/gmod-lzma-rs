#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "linux")]
pub type size_t = ::std::os::raw::c_ulong;

#[cfg(target_os = "windows")]
pub type size_t = ::std::os::raw::c_ulonglong;

extern "C" {
	pub fn LzmaCompress(
		dest: *mut ::std::os::raw::c_uchar,
		destLen: *mut size_t,
		src: *const ::std::os::raw::c_uchar,
		srcLen: size_t,
		outProps: *mut ::std::os::raw::c_uchar,
		outPropsSize: *mut size_t,
		level: ::std::os::raw::c_int,
		dictSize: ::std::os::raw::c_uint,
		lc: ::std::os::raw::c_int,
		lp: ::std::os::raw::c_int,
		pb: ::std::os::raw::c_int,
		fb: ::std::os::raw::c_int,
		numThreads: ::std::os::raw::c_int,
	) -> ::std::os::raw::c_int;
}
extern "C" {
	pub fn LzmaUncompress(
		dest: *mut ::std::os::raw::c_uchar,
		destLen: *mut size_t,
		src: *const ::std::os::raw::c_uchar,
		srcLen: *mut size_t,
		props: *const ::std::os::raw::c_uchar,
		propsSize: size_t,
	) -> ::std::os::raw::c_int;
}
