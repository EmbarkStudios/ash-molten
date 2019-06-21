use std::path::PathBuf;
// Features are not used inside build scripts, so we have to explicitly query them from the
// enviroment
fn is_external_enabled() -> bool {
    std::env::vars()
        .filter_map(|(flag, _)| {
            const NAME: &'static str = "CARGO_FEATURE_";
            if flag.starts_with(NAME) {
                let feature = flag.split(NAME).nth(1).expect("").to_string();
                println!("{:?}", feature);
                return Some(feature);
            }
            None
        })
        .find(|f| f == "EXTERNAL")
        .is_some()
}
fn main() {
    if !(cfg!(target = "macos") || cfg!(target = "ios")) {
        panic!("ash-molten can only be built on macOS or of iOS");
    }
    // The 'external' feature was not enabled. Molten will be built automaticaly.
    if !is_external_enabled() {
        std::process::Command::new("bash")
            .arg("build_molten.sh")
            .status()
            .expect("Unable to build molten");
        let project_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("native");
        println!("cargo:rustc-link-search=native={}", project_dir.display());
    }
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=IOSurface");
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=static=MoltenVK");
}
