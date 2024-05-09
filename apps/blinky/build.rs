use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // put linker script in our output directory and ensure it's on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("../../memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // rebuild if the linker script changes
    println!("cargo:rerun-if-changed=memory.x");
}
