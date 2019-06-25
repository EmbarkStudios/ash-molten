#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac {
    use std::path::{Path, PathBuf};

    // Features are not used inside build scripts, so we have to explicitly query them from the
    // enviroment
    pub(crate) fn is_external_enabled() -> bool {
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

    pub(crate) fn build_molten<P: AsRef<Path>>(target_dir: &P) -> &'static str {
        use std::{
            process::Command,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
        };

        let exit = Arc::new(AtomicBool::new(false));
        let wants_exit = exit.clone();

        // Periodically emit log messages so that Travis doesn't make a sad
        let handle = std::thread::spawn(move || {
            let mut counter = 0;
            while !wants_exit.load(Ordering::Acquire) {
                std::thread::sleep(std::time::Duration::from_millis(100));
                counter += 100;

                if counter >= 30 * 1000 {
                    counter = 0;
                    println!("still building MoltenVK");
                }
            }
        });

        let git_status = if Path::new("MoltenVK").exists() {
            Command::new("git")
                .arg("pull")
                .spawn()
                .expect("failed to spawn git")
                .wait()
                .expect("failed to pull MoltenVK")
        } else {
            Command::new("git")
                .arg("clone")
                .arg("https://github.com/KhronosGroup/MoltenVK.git")
                .spawn()
                .expect("failed to spawn git")
                .wait()
                .expect("failed to clone MoltenVK")
        };

        assert!(git_status.success(), "failed to clone MoltenVK");

        let status = Command::new("sh")
            .current_dir("MoltenVK")
            .arg("fetchDependencies")
            .spawn()
            .expect("failed to spawn fetchDependencies")
            .wait()
            .expect("failed to fetchDependencies");

        assert!(status.success(), "failed to fetchDependencies");

        // These (currently) match the identifiers used by moltenvk
        let (target_name, dir) = match std::env::var("CARGO_CFG_TARGET_OS") {
            Ok(target) => match target.as_ref() {
                "macos" => ("macos", "macOS"),
                "ios" => ("ios", "iOS"),
                target => panic!("unknown target '{}'", target),
            },
            Err(e) => panic!("failed to determinte target os '{}'", e),
        };

        let status = Command::new("make")
            .current_dir("MoltenVK")
            .arg(target_name)
            .spawn()
            .expect("failed to spawn fetchDependencies")
            .wait()
            .expect("failed to fetchDependencies");

        assert!(status.success(), "failed to fetchDependencies");

        let src = {
            let mut pb = PathBuf::new();
            pb.push("MoltenVK");
            pb.push("Package/Release/MoltenVK");
            pb.push(dir);
            pb.push("static/libMoltenVK.a");
            pb
        };

        let target = {
            let mut pb = PathBuf::new();
            pb.push(target_dir);
            pb.push(format!("lib{}MoltenVK.a", target_name));
            pb
        };

        if let Err(e) = std::fs::copy(&src, &target) {
            panic!("failed to copy {:?} to {:?}: {}", src, target, e);
        }

        exit.store(true, Ordering::Release);
        handle.join().unwrap();
        target_name
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    use crate::mac::*;
    use std::path::{Path, PathBuf};

    // The 'external' feature was not enabled. Molten will be built automaticaly.
    let target_name = if !is_external_enabled() {
        let target_dir = Path::new("native");

        let target_name = build_molten(&target_dir);
        let project_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
        )
        .join(target_dir);
        println!("cargo:rustc-link-search=native={}", project_dir.display());

        target_name
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else {
        unreachable!();
    };

    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=IOSurface");
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=static={}MoltenVK", target_name);
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    eprintln!("ash-molten requires either 'macos' or 'ios' target");
}
