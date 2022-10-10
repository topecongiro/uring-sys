use std::env;
use std::fs;
use std::path::*;
use std::process::Command;

use cc::Build;

fn main() {
    let project = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let liburing = project.join("liburing");

    // Run the configure script in OUT_DIR to get `compat.h`
    let configured_include = configure(&liburing);

    let src = liburing.join("src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // liburing
    Build::new()
        .file(src.join("setup.c"))
        .file(src.join("queue.c"))
        .file(src.join("syscall.c"))
        .file(src.join("register.c"))
        .file(src.join("version.c"))
        .define("_DEFAULT_SOURCE", "")
        .include(&configured_include)
        .extra_warnings(false)
        .compile("uring");

    let bindings = bindgen::Builder::default()
        .header(src.join("include").join("liburing.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file("^(.*liburing.h)$")
        .allowlist_file("^(.*io_uring.h)$")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("rusturing.c"))
        .rustified_enum("io_uring_op") // Not used in C code; safe to rustify
        .rustified_non_exhaustive_enum("io_uring_op")
        .generate()
        .unwrap();

    // (our additional, linkable C bindings)
    Build::new()
        .file(out_dir.join("rusturing.c"))
        .include(&configured_include)
        .compile("rusturing");

    let out_path = out_dir.join("bindings.rs");
    bindings.write_to_file(out_path).unwrap();

    println!("cargo:include={}", configured_include.display());
}

fn configure(liburing: &Path) -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap();
    fs::create_dir_all(out_dir.join("src/include/liburing")).unwrap();
    fs::copy(liburing.join("configure"), out_dir.join("configure")).unwrap();
    fs::copy(liburing.join("src/include/liburing.h"), out_dir.join("src/include/liburing.h")).unwrap();
    fs::copy(liburing.join("src/include/liburing/barrier.h"), out_dir.join("src/include/liburing/barrier.h")).unwrap();
    fs::copy(liburing.join("src/include/liburing/io_uring.h"), out_dir.join("src/include/liburing/io_uring.h")).unwrap();
    fs::copy(
        liburing.join("Makefile.common"),
        out_dir.join("Makefile.common"),
    )
    .unwrap();
    fs::copy(
        liburing.join("liburing.spec"),
        out_dir.join("liburing.spec"),
    )
    .unwrap();
    let ret = Command::new("./configure")
        .current_dir(&out_dir)
        .output()
        .unwrap();
    if !ret.status.success() {
        panic!(
            "configure failed: {}",
            String::from_utf8(ret.stderr).unwrap_or_default()
        );
    }
    out_dir.join("src/include")
}
