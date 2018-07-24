extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut wrapper: File = File::create("wrapper.h").expect("Failed to create file");

    #[cfg(feature = "cdio")]
    wrapper
        .write_all(b"#include \"cdio.h\"\n")
        .expect("Failed to write to file");

    #[cfg(feature = "iso9660")]
    wrapper
        .write_all(b"#include \"iso9660.h\"\n")
        .expect("Failed to write to file");

    #[cfg(feature = "udf")]
    wrapper
        .write_all(b"#include \"udf.h\"\n")
        .expect("Failed to write to file");

    wrapper.flush().expect("Failed to write to file");

    #[cfg(feature = "cdio")]
    println!("cargo:rustc-link-lib=cdio");

    #[cfg(feature = "iso9660")]
    println!("cargo:rustc-link-lib=iso9660");

    #[cfg(feature = "udf")]
    println!("cargo:rustc-link-lib=udf");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Fix error E0133 (see https://github.com/rust-lang/rust/issues/46043)
        .derive_debug(false)
        .derive_copy(cfg!(not(feature = "docsrs-workaround")))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
