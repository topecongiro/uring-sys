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

    // liburing
    Build::new()
        .file(src.join("setup.c"))
        .file(src.join("queue.c"))
        .file(src.join("syscall.c"))
        .file(src.join("register.c"))
        .define("_DEFAULT_SOURCE", "")
        .include(&configured_include)
        .extra_warnings(false)
        .compile("uring");

    // (our additional, linkable C bindings)
    Build::new()
        .file(project.join("rusturing.c"))
        .include(&configured_include)
        .compile("rusturing");

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
    Command::new("./configure")
        .current_dir(&out_dir)
        .output()
        .expect("configure script failed");
    out_dir.join("src/include")
}
