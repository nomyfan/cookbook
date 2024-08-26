// build.rs
extern crate bindgen;

use std::env;

fn main() {
    let qjs_sys = env::current_dir().unwrap().join("quickjs-sys");
    let qjs = qjs_sys.join("quickjs");

    // Compile the wrapper
    std::process::Command::new("clang")
        .args([
            "-O",
            "-c",
            "-o",
            qjs_sys.join("quickjswrap.o").to_str().unwrap(),
            qjs_sys.join("quickjswrap.c").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    std::process::Command::new("ar")
        .args([
            "rcs",
            qjs_sys.join("libquickjswrap.a").to_str().unwrap(),
            qjs_sys.join("quickjswrap.o").to_str().unwrap(),
        ])
        .output()
        .unwrap();

    println!("cargo:rustc-link-lib=static=quickjs");
    println!("cargo:rustc-link-search=native={}", qjs.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=quickjswrap");
    println!(
        "cargo:rustc-link-search=native={}",
        qjs_sys.to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(qjs.join("quickjs.h").to_str().unwrap())
        .header(qjs_sys.join("quickjswrap.h").to_str().unwrap())
        .generate()
        .unwrap();

    bindings.write_to_file(qjs_sys.join("bindings.rs")).unwrap();
}
