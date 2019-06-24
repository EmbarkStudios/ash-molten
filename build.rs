use std::path::PathBuf;
// Iterator over target_os cfg flags
fn target_os() -> impl Iterator<Item = String> {
    std::env::vars().filter_map(|(flag, val)| {
        const NAME: &'static str = "CARGO_CFG_TARGET_OS";
        if flag.starts_with(NAME) {
            Some(val)
        } else {
            None
        }
    })
}
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
    let supported_platforms = ["macos", "ios"];
    let is_mac_or_ios = target_os()
        .find(|target| supported_platforms.contains(&target.as_str()))
        .is_some();
    if !is_mac_or_ios {
        panic!("ash-molten can only be built on macOS or of iOS");
    }
    // The 'external' feature was not enabled. Molten will be built automaticaly.
    if !is_external_enabled() {
        let mut build = std::process::Command::new("bash")
            .arg("build_molten.sh")
            .spawn()
            .expect("Unable to build molten");
        while build.try_wait().unwrap().is_none() {
            println!("Still building MoltenVK");
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
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
