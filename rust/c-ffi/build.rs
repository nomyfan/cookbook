fn main() {
    let bindings = bindgen::builder()
        .header("c-src/include/ffi.h")
        .generate()
        .unwrap();

    bindings.write_to_file("src/ffi.rs").unwrap();

    cc::Build::new()
        .include("c-src/include")
        .file("c-src/ffi.c")
        .compile("ffi_sys");
}
