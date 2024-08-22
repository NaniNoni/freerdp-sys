use std::{env, path::PathBuf};

fn main() {
    let freerdp = pkg_config::Config::new()
        .statik(true)
        .atleast_version("2.0")
        .probe("freerdp3");

    if let Ok(lib) = freerdp {
        for link_file in lib.link_files {
            println!("cargo::rustc-link-lib=static={:?}", link_file);
        }
    }

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // TODO: Find a better solution. This is required as the freerdp header expects both the
        // freerdp and winpr folders to be in the include path, which they are not. On both Ubuntu
        // 24.04 and Arch Linux they were located at '/usr/include/freerdp3/freerdp/' and '/usr/include/winpr3/winpr'
        .clang_arg("-I/usr/include/winpr3")
        .clang_arg("-I/usr/include/freerdp3")
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
