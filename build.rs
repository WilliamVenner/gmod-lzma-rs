fn main() {
	println!("cargo:rerun-if-changed=bootil");
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=CARGO_BUILD_TARGET");

	// Build Bootil LZMA
	cc::Build::new()
		.file("bootil/src/3rdParty/lzma/LzFind.c")
		.file("bootil/src/3rdParty/lzma/LzmaLib.c")
		.file("bootil/src/3rdParty/lzma/LzmaDec.c")
		.file("bootil/src/3rdParty/lzma/LzmaEnc.c")
		.file("bootil/src/3rdParty/lzma/Alloc.c")
		.compile("lzma");
}
