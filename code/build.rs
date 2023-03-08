extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use pkg_config::Library;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=libSaleaeLogic");

    let dst = Config::new("libSaleaeLogic").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=saleaeLogic");

    let mut vec: Vec<Library> = Vec::new();
    vec.push(pkg_config::probe_library("glib-2.0").unwrap());
    vec.push(pkg_config::probe_library("libsigrok").unwrap());
    pkg_config::probe_library("libsigrokdecode").unwrap();

    let mut builder = bindgen::Builder::default().header("libSaleaeLogic/wrapper.h");
    let paths = vec
        .iter()
        .flat_map(|lib| lib.include_paths.to_vec());
    for path in paths {
        builder = builder.clang_arg(format!("-I{}", path.to_str().unwrap()));    // TODO include this in map :)
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
