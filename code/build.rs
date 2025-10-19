use std::path::Path;
use std::{env, path::PathBuf};
use std::{
    fs::File,
    io::{Read, Write},
};

#[cfg(feature = "stm32f411")]
const MEMORY_X_FILE: &str = "memory-configs/memory_f411.x";
#[cfg(feature = "stm32h723")]
const MEMORY_X_FILE: &str = "memory-configs/memory_h723.x";
#[cfg(feature = "stm32h743")]
const MEMORY_X_FILE: &str = "memory-configs/memory_h743.x";

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    #[cfg(feature = "rtt-logging")]
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    copy_memory_x(out_dir);

    println!("cargo:rustc-link-search={}", out_dir.display());
}

fn copy_memory_x(out_dir: &Path) {
    println!("cargo::rerun-if-changed={MEMORY_X_FILE}");

    let mut source_content: Vec<u8> = Vec::default();

    let mut source_file = File::open(MEMORY_X_FILE).unwrap();
    source_file.read_to_end(&mut source_content).unwrap();

    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(&source_content)
        .unwrap();
}
