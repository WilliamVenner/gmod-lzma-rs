#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type size_t = libc::size_t;

#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
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

	pub fn LzmaUncompress(
		dest: *mut ::std::os::raw::c_uchar,
		destLen: *mut size_t,
		src: *const ::std::os::raw::c_uchar,
		srcLen: *mut size_t,
		props: *const ::std::os::raw::c_uchar,
		propsSize: size_t,
	) -> ::std::os::raw::c_int;
}

#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
extern "stdcall" {
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

	pub fn LzmaUncompress(
		dest: *mut ::std::os::raw::c_uchar,
		destLen: *mut size_t,
		src: *const ::std::os::raw::c_uchar,
		srcLen: *mut size_t,
		props: *const ::std::os::raw::c_uchar,
		propsSize: size_t,
	) -> ::std::os::raw::c_int;
}
