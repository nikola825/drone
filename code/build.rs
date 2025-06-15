use std::path::Path;
use std::{env, path::PathBuf};
use std::{fs::File, io::Write};

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    copy_memory_x(out_dir);

    println!("cargo:rustc-link-search={}", out_dir.display());
}

#[cfg(feature = "stm32h723")]
fn copy_memory_x(out_dir: &Path) {
    println!("cargo::rerun-if-changed=memory-configs/memory_h723.x");

    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory-configs/memory_h723.x"))
        .unwrap();
}

#[cfg(feature = "stm32f411")]
fn copy_memory_x(out_dir: &Path) {
    println!("cargo::rerun-if-changed=memory-configs/memory_f411.x");

    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory-configs/memory_f411.x"))
        .unwrap();
}

#[cfg(feature = "stm32h743")]
fn copy_memory_x(_: &Path) {
    // println!("cargo::rerun-if-changed=memory-configs/memory_f411.x");

    /*File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory-configs/memory_f411.x"))
        .unwrap();*/
}
