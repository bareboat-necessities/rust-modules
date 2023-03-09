extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=../../RTIMULib/build/");
    println!("cargo:rustc-link-lib=RTIMULib");

    let bindings = bindgen::Builder::default()
        .allowlist_type("RT.*")        
        .allowlist_type("Fusion.*")        
        .header("wrapper.hpp")
        //.clang_arg("-x=c++")
        .clang_arg("-std=c++11")
        .clang_arg("-I../../RTIMULib/")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

