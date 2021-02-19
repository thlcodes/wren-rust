use std::env;
use std::path::Path;
use std::process::Command;

#[allow(dead_code)]
fn make_debug(dir: &Path, carg: &str) {
    let status = Command::new("make").current_dir(dir).arg("-C").arg(carg).arg("config=debug_64bit").status();
    assert!(status.unwrap().success());
    println!("cargo:rustc-link-lib=static=wren_d");
}

#[allow(dead_code)]
fn make_release(dir: &Path, carg: &str) {
    let status = Command::new("make").current_dir(dir).arg(carg).arg("config=release_64bit").status();
    assert!(status.unwrap().success());
    println!("cargo:rustc-link-lib=static=wren");
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let wren_make_dir = manifest_path.join("wren");
    let wren_lib_dir = manifest_path.join("wren/lib");
    let mut carg = "projects/make/";
    if env::consts::OS == "macos" {
        carg = "projects/make.mac/";
    }

    #[cfg(debug_assertions)]
    make_debug(&wren_make_dir, carg);
    #[cfg(not(debug_assertions))]
    make_release(&wren_make_dir, carg);

    println!("cargo:rustc-link-search=native={}", wren_lib_dir.display());
}
