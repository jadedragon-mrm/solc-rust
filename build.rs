extern crate bindgen;
extern crate cmake;

use cmake::Config;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("solidity")
        .define("TESTS", "OFF")
        .define("TOOLS", "OFF")
        .define("USE_Z3", "OFF")
        .define("USE_CVC4", "OFF")
        .define("Boost_USE_STATIC_LIBS", "ON")
        .define("MAKE_C_COMPILER", "/usr/bin/c++-11")
        .define("CMAKE_CXX_COMPILER", "/usr/bin/g++-11")
        .build();

    /*for lib in vec!["solc", "solidity", "yul", "langutil", "evmasm", "solutil", "smtutil"] {
        println!(
            "cargo:rustc-link-search=native={}/build/lib{}",
            dst.display(),
            lib
        );
        println!("cargo:rustc-link-lib=static={}", lib);
    }*/

    for lib in vec!["solc"] {
        println!(
            "cargo:rustc-link-search=native={}/build/lib{}",
            dst.display(),
            lib
        );
        println!("cargo:rustc-link-lib={}", lib);
    }

    // println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // println!("cargo:rustc-link-search=native=/usr/lib/");
    // println!("cargo:rustc-link-search=native=/usr/lib64/");
    // println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    // println!("cargo:rustc-link-search=native=/usr/local/lib/");

    //println!("cargo:rustc-link-lib=static=boost_system");
    //println!("cargo:rustc-link-lib=static=boost_filesystem");
    //println!("cargo:rustc-link-lib=static=boost_regex");

    // println!("cargo:rustc-link-lib=boost_system");
    // println!("cargo:rustc-link-lib=boost_filesystem");
    // println!("cargo:rustc-link-lib=boost_regex");

    // We need to link against C++ std lib
    // if let Some(cpp_stdlib) = get_cpp_stdlib() {
    //     println!("cargo:rustc-link-lib={}", cpp_stdlib);
    // }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("solidity/libsolc/libsolc.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// See https://github.com/alexcrichton/gcc-rs/blob/88ac58e25/src/lib.rs#L1197
/*fn get_cpp_stdlib() -> Option<String> {
    env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}*/
