use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let frontend_dir = path.join("frontend");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=FRONTEND_DIR={}", frontend_dir.display());
}
