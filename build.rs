extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn gen_bindings(name: &str) {
    let bindings = bindgen::Builder::default()
        .header(format!("bindings/{}.h", name))
        .generate()
        .expect("Unable to generate");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Failed to fetch env"));
    bindings
        .write_to_file(out_path.join(format!("bindings_{}.rs", name)))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rustc-link-lib=input");
    println!("cargo:rustc-link-lib=udev");

    gen_bindings("fcntl");
    gen_bindings("libinput");
    gen_bindings("udev");
    gen_bindings("unistd");
}
