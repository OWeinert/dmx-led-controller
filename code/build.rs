extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use pkg_config::Library;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=libLogicAnalyzer");

    let dst = Config::new("libLogicAnalyzer").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=logicAnalyzer");

    let mut vec: Vec<Library> = Vec::new();
    vec.push(pkg_config::probe_library("glib-2.0").unwrap());
    vec.push(pkg_config::probe_library("libsigrok").unwrap());
    vec.push(pkg_config::probe_library("libsigrokdecode").unwrap());

    let mut builder = bindgen::Builder::default().header("libLogicAnalyzer/libLogicAnalyzer.h");
    let paths = vec.iter().flat_map(|lib| lib.include_paths.to_vec());
    for path in paths {
        builder = builder.clang_arg(format!("-I{}", path.to_str().unwrap()));
    }
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
