use std::path::PathBuf;
fn main() {
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("external");
    println!("cargo:rustc-link-search=native={}", project_dir.display());
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=IOSurface");
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=static=MoltenVK");
}
