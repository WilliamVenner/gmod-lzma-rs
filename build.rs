fn main() {
	println!("cargo:rerun-if-changed=bootil");
	println!("cargo:rerun-if-changed=build.rs");

	// Build Bootil LZMA
	cc::Build::new()
		.file("bootil/src/3rdParty/lzma/LzFind.c")
		.file("bootil/src/3rdParty/lzma/LzmaLib.c")
		.file("bootil/src/3rdParty/lzma/LzmaDec.c")
		.file("bootil/src/3rdParty/lzma/LzmaEnc.c")
		.file("bootil/src/3rdParty/lzma/Alloc.c")
		.compile("lzma");

	// Generate bindings
	let bindings = bindgen::Builder::default()
		.header("bootil/src/3rdParty/lzma/Types.h")
		.header("bootil/src/3rdParty/lzma/LzmaLib.h")
		.allowlist_function("LzmaCompress")
		.allowlist_function("LzmaUncompress")
		.default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
