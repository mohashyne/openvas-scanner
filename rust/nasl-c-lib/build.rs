use std::fs::canonicalize;

fn main() {
    println!(
        "cargo:rustc-link-search={}",
        canonicalize("./lib").unwrap().to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=gcrypt");
    println!("cargo:rustc-link-lib=static=gpg-error");
    println!("cargo:rerun-if-changed=c/cryptographic/gcrypt_mac.h");
    println!("cargo:rerun-if-changed=c/cryptographic/gcrypt_error.h");
    println!("cargo:rerun-if-changed=lib/libgcrypt.a");
    println!("cargo:rerun-if-changed=lib/libgpg-error.a");

    cc::Build::new()
        .file("c/cryptographic/gcrypt_mac.c")
        .file("c/cryptographic/gcrypt_error.c")
        .include(canonicalize("./include").unwrap())
        .compile("crypt");
}
