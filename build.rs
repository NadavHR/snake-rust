use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=c_functions/c_functions.c");
    println!("cargo::rerun-if-changed=c_functions/c_functions.h");
    let output = Command::new("make").args([ "-C", "c_functions"]).output().expect("failed to build c_functions");
    println!("cargo::warning={}", String::from_utf8(output.stdout).unwrap());
    println!("cargo::rustc-link-search=native=lib");
    println!("cargo::rustc-link-lib=static=c_functions");
    println!("cargo::rustc-link-search=native=c_functions/vcpkg_installed/x64-windows/lib");
    println!("cargo::rustc-link-lib=static=SDL2");
}