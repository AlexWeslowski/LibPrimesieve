fn main() {
    println!("cargo:rustc-link-lib=primesieve");
    println!("cargo:rerun-if-changed=build.rs");
}
