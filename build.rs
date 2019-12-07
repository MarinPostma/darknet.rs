use std::env;
use std::path::PathBuf;

fn main() {
    // trim this, how much of htis is really necessary??
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=darknet/");
    println!("cargo:rustc-link-lib=static=darknet");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=cublas");
    println!("cargo:rustc-link-lib=dylib=curand");
    println!("cargo:rustc-link-search=darknet");
    println!("cargo:rustc-link-search=darknet");
    println!("cargo:rustc-link-search=/opt/cuda/lib");
    println!("cargo:rustc-link-search=/opt/cuda/lib64");
    println!("cargo:rustc-link-search=/opt/cuda/nvvm/lib");
    println!("cargo:rustc-link-search=/opt/cuda/nvvm/lib64");
    println!("cargo:rustc-link-search=/usr/local/cuda/lib");

    let bindings = bindgen::Builder::default()
        .header("darknet/include/darknet.h")
        .no_copy("detection")
        .generate()
        .expect("unable to generate darknet bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write darknet bindings");
}
