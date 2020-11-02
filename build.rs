use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let dst = Config::new("vendor/Utilities/gdcmjpeg")
        .define("GDCM_VERSION", "3.0.8")
        .define("GDCM_MAJOR_VERSION", "3")
        .define("GDCM_MINOR_VERSION", "0")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=jpeg8");
    println!("cargo:rustc-link-lib=static=jpeg12");
    println!("cargo:rustc-link-lib=static=jpeg16");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    for bits in ["8", "12", "16"].iter() {
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header(format!("wrapper{}.h", bits))
            .clang_arg(&format!("-I{}/include", out_path.display()))
            .clang_arg(&format!("-L{}/lib", out_path.display()))
            //.clang_arg(&format!("-I{}/build/12", dst.display()))
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
    
        bindings
            .write_to_file(out_path.join(format!("bindings{}.rs", bits)))
            .expect("Couldn't write bindings!");
    
        let manifestpath = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        bindings
            .write_to_file(manifestpath.join(format!("src/bindings{}.rs", bits)))
            .expect("Couldn't write bindings!");
    }
}