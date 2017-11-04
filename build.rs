extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=udev");

    let bindings = bindgen::Builder::default()
        .header("bindings/udev.h")
        .generate()
        .expect("Unable to generate");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings_udev.rs"))
        .expect("Couldn't write bindings!");
}
