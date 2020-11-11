#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac {
    use std::path::Path;

    // MoltenVK git tagged release to use
    pub static VERSION: &str = "1.1.0";

    // Features are not used inside build scripts, so we have to explicitly query them from the
    // enviroment
    pub(crate) fn is_feature_enabled(feature: &str) -> bool {
        std::env::vars()
            .filter_map(|(flag, _)| {
                const NAME: &str = "CARGO_FEATURE_";
                if flag.starts_with(NAME) {
                    let feature = flag.split(NAME).nth(1).expect("").to_string();
                    println!("{:?}", feature);
                    return Some(feature);
                }
                None
            })
            .any(|f| f == feature)
    }

    pub(crate) fn build_molten<P: AsRef<Path>>(_target_dir: &P) -> &'static str {
        use std::{
            process::Command,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
        };

        let checkout_dir = Path::new(&std::env::var("OUT_DIR").expect("Couldn't find OUT_DIR"))
            .join(format!("MoltenVK-{}", VERSION));

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

        let git_status = if Path::new(&checkout_dir).exists() {
            Command::new("git")
                .current_dir(&checkout_dir)
                .arg("pull")
                .spawn()
                .expect("failed to spawn git")
                .wait()
                .expect("failed to pull MoltenVK")
        } else {
            Command::new("git")
                .arg("clone")
                .arg("--branch")
                .arg(format!("v{}", VERSION.to_owned()))
                .arg("--depth")
                .arg("1")
                .arg("https://github.com/KhronosGroup/MoltenVK.git")
                .arg(&checkout_dir)
                .spawn()
                .expect("failed to spawn git")
                .wait()
                .expect("failed to clone MoltenVK")
        };

        assert!(git_status.success(), "failed to get MoltenVK");

        // These (currently) match the identifiers used by moltenvk
        let (target_name, _dir) = match std::env::var("CARGO_CFG_TARGET_OS") {
            Ok(target) => match target.as_ref() {
                "macos" => ("macos", "macOS"),
                "ios" => ("ios", "iOS"),
                target => panic!("unknown target '{}'", target),
            },
            Err(e) => panic!("failed to determinte target os '{}'", e),
        };

        let status = Command::new("sh")
            .current_dir(&checkout_dir)
            .arg("fetchDependencies")
            .arg(format!("--{}", target_name))
            .spawn()
            .expect("failed to spawn fetchDependencies")
            .wait()
            .expect("failed to fetchDependencies");

        assert!(status.success(), "failed to fetchDependencies");

        let status = Command::new("make")
            .current_dir(&checkout_dir)
            .arg(target_name)
            .spawn()
            .expect("failed to build MoltenVK")
            .wait()
            .expect("failed to build MoltenVK");

        assert!(status.success(), "failed to build MoltenVK");

        exit.store(true, Ordering::Release);
        handle.join().unwrap();
        target_name
    }

    pub(crate) fn download_prebuilt_molten<P: AsRef<Path>>(target_dir: &P) {
        use std::process::{Command, Stdio};

        std::fs::create_dir_all(&target_dir).expect("Couldn't create directory");

        std::env::set_current_dir(&target_dir).expect("Couldn't change current directory");

        let curl = Command::new("curl")
            .arg("-s")
            .arg(format!(
                "https://api.github.com/repos/EmbarkStudios/ash-molten/releases/tags/MoltenVK-{}",
                VERSION
            ))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't launch curl");

        let curl_out = curl.stdout.expect("Failed to open curl stdout");

        let grep = Command::new("grep")
            .arg("browser_download_url.*zip")
            .stdin(Stdio::from(curl_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't launch grep");

        let grep_out = grep.stdout.expect("Failed to open grep stdout");

        let cut = Command::new("cut")
            .args(&["-d", ":", "-f", "2,3"])
            .stdin(Stdio::from(grep_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't launch cut");

        let cut_out = cut.stdout.expect("Failed to open grep stdout");

        let tr = Command::new("tr")
            .args(&["-d", "\""])
            .stdin(Stdio::from(cut_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't launch cut");

        let tr_out = tr.stdout.expect("Failed to open grep stdout");

        let output = Command::new("xargs")
            .args(&["-n", "1", "curl", "-LO", "--silent"])
            .stdin(Stdio::from(tr_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't launch cut")
            .wait_with_output()
            .expect("failed to wait on xargs");

        assert!(output.status.success());

        for path in std::fs::read_dir(&target_dir).expect("Couldn't read dir") {
            let path = path.unwrap().path();
            if let Some("zip") = path.extension().and_then(std::ffi::OsStr::to_str) {
                let status = Command::new("unzip")
                    .arg("-o")
                    .arg(path.to_owned())
                    .arg("-x")
                    .arg("__MACOSX/*")
                    .spawn()
                    .expect("Couldn't launch unzip")
                    .wait()
                    .expect("failed to wait on unzip");

                assert!(status.success());
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    use crate::mac::*;
    use std::path::{Path, PathBuf};

    // The 'external' feature was not enabled. Molten will be built automaticaly.
    let external_enabled = is_feature_enabled("EXTERNAL");
    let pre_built_enabled = is_feature_enabled("PRE_BUILT");

    assert!(
        !(external_enabled && pre_built_enabled),
        "external and prebuild cannot be active at the same time"
    );

    if !external_enabled && !pre_built_enabled {
        let target_dir = Path::new(&std::env::var("OUT_DIR").unwrap())
            .join(format!("MoltenVK-{}", crate::mac::VERSION,));
        let _target_name = build_molten(&target_dir);

        let project_dir = {
            let mut pb = PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
            );
            pb.push(target_dir);
            pb.push(format!(
                "Package/Latest/MoltenVK/MoltenVK.xcframework/{}-{}",
                std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
                std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
            ));

            pb
        };

        println!("cargo:rustc-link-search=native={}", project_dir.display());
    } else if pre_built_enabled {
        let target_dir = Path::new(&std::env::var("OUT_DIR").unwrap())
            .join(format!("Prebuilt-MoltenVK-{}", crate::mac::VERSION));

        download_prebuilt_molten(&target_dir);

        let project_dir = {
            let mut pb = PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
            );
            pb.push(target_dir);
            pb.push(format!(
                "{}-{}",
                std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
                std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
            ));

            pb
        };

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

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    eprintln!("ash-molten requires either 'macos' or 'ios' target");
}
