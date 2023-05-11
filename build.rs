extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    // build c-xs233.
    // the build directives are based on the makefile.
    cc::Build::new()
        .file("./c-xs233/xsk233.c")
        .extra_warnings(true)
        .flag("-Wundef")
        .flag("-Wshadow")
        .flag("-O2")
        .flag("-mpclmul")
        .flag("-mavx2")
        .compile("xsk233");

    cc::Build::new()
        .file("./c-xs233/xsb233.c")
        .extra_warnings(true)
        .flag("-Wundef")
        .flag("-Wshadow")
        .flag("-O2")
        .flag("-mpclmul")
        .flag("-mavx2")
        .compile("xsb233");

    // generate bindings
    bindgen::Builder::default()
        .header("c-xs233/xs233.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings");
}
