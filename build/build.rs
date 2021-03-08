mod xcframework;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac {

    use std::path::Path;

    // MoltenVK git tagged release to use
    pub static MOLTEN_VK_VERSION: &str = "1.1.2";
    pub static MOLTEN_VK_PATCH: Option<&str> = Some("f28ab1c");

    // Return the artifact tag in the form of "x.x.x" or if there is a patch specified "x.x.x#yyyyyyy"
    pub(crate) fn get_artifact_tag() -> String {
        if let Some(patch) = MOLTEN_VK_PATCH {
            format!("{}#{}", MOLTEN_VK_VERSION, patch)
        } else {
            MOLTEN_VK_VERSION.to_owned()
        }
    }

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
            .join(format!("MoltenVK-{}", get_artifact_tag()));

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

        if Path::new(&checkout_dir).exists() {
            // Don't pull if a specific hash has been checkedout
            if MOLTEN_VK_PATCH.is_none() {
                let git_status = Command::new("git")
                    .current_dir(&checkout_dir)
                    .arg("pull")
                    .spawn()
                    .expect("failed to spawn git")
                    .wait()
                    .expect("failed to pull MoltenVK");

                assert!(git_status.success(), "failed to get MoltenVK");
            }
        } else {
            let branch = format!("v{}", MOLTEN_VK_VERSION.to_owned());
            let clone_args = if MOLTEN_VK_PATCH.is_none() {
                vec!["--branch", branch.as_str(), "--depth", "1"]
            } else {
                vec!["--single-branch", "--branch", "master"] // Can't specify depth if you switch to a different commit hash later.
            };
            let git_status = Command::new("git")
                .arg("clone")
                .args(clone_args)
                .arg("https://github.com/KhronosGroup/MoltenVK.git")
                .arg(&checkout_dir)
                .spawn()
                .expect("failed to spawn git")
                .wait()
                .expect("failed to clone MoltenVK");

            assert!(git_status.success(), "failed to get MoltenVK");
        };

        if let Some(patch) = MOLTEN_VK_PATCH {
            let git_status = Command::new("git")
                .current_dir(&checkout_dir)
                .arg("checkout")
                .arg(patch)
                .status()
                .expect("failed to spawn git");

            assert!(git_status.success(), "failed to checkout patch");
        }

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

        let previous_path = std::env::current_dir().expect("Couldn't get current directory");

        std::env::set_current_dir(&target_dir).expect("Couldn't change current directory");

        let curl = Command::new("curl")
            .arg("-s")
            .arg(format!(
                "https://api.github.com/repos/EmbarkStudios/ash-molten/releases/tags/MoltenVK-{}",
                get_artifact_tag().replace("#", "%23")
            ))
            .stdout(Stdio::piped())
            .status()
            .expect("Couldn't launch curl");

        let curl_out = curl.stdout.expect("Failed to open curl stdout");

        let grep = Command::new("grep")
            .arg("browser_download_url.*zip")
            .stdin(Stdio::from(curl_out))
            .stdout(Stdio::piped())
            .status()
            .expect("Couldn't launch grep");

        let grep_out = grep.stdout.expect("Failed to open grep stdout");

        let cut = Command::new("cut")
            .args(&["-d", ":", "-f", "2,3"])
            .stdin(Stdio::from(grep_out))
            .stdout(Stdio::piped())
            .status()
            .expect("Couldn't launch cut");

        let cut_out = cut.stdout.expect("Failed to open grep stdout");

        let tr = Command::new("tr")
            .args(&["-d", "\""])
            .stdin(Stdio::from(cut_out))
            .stdout(Stdio::piped())
            .status()
            .expect("Couldn't launch tr");

        let tr_out = tr.stdout.expect("Failed to open grep stdout");

        let output = Command::new("xargs")
            .args(&["-n", "1", "curl", "-LO", "--silent"])
            .stdin(Stdio::from(tr_out))
            .stdout(Stdio::piped())
            .status()
            .expect("Couldn't launch xargs")
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
                    .status()
                    .expect("Couldn't launch unzip")
                    .wait()
                    .expect("failed to wait on unzip");

                assert!(status.success());
            }
        }

        std::env::set_current_dir(&previous_path).expect("Couldn't change current directory");
    }
}

use std::{
    collections::{hash_map::RandomState, HashMap},
    path::{Path, PathBuf},
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    use crate::mac::*;
    // The 'external' feature was not enabled. Molten will be built automaticaly.
    let external_enabled = is_feature_enabled("EXTERNAL");
    let pre_built_enabled = is_feature_enabled("PRE_BUILT");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    assert!(
        !(external_enabled && pre_built_enabled),
        "external and prebuild cannot be active at the same time"
    );

    if !external_enabled {
        let mut project_dir = if pre_built_enabled {
            let target_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join(format!(
                "Prebuilt-MoltenVK-{}",
                crate::mac::get_artifact_tag()
            ));

            download_prebuilt_molten(&target_dir);

            let mut pb = PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
            );
            pb.push(target_dir);
            pb.push("MoltenVK.xcframework");
            pb
        } else {
            let target_dir = Path::new(&std::env::var("OUT_DIR").unwrap())
                .join(format!("MoltenVK-{}", crate::mac::get_artifact_tag()));
            let _target_name = build_molten(&target_dir);

            let mut pb = PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
            );
            pb.push(target_dir);
            pb.push("Package/Latest/MoltenVK/MoltenVK.xcframework");

            pb
        };

        let xcframework =
            xcframework::XcFramework::parse(&project_dir).expect("Failed to parse XCFramework");
        let native_libs = xcframework
            .AvailableLibraries
            .into_iter()
            .map(|lib| {
                lib.universal_to_native(project_dir.clone())
                    .expect("Failed to get native library")
            })
            .flatten()
            .map(|lib| (lib.identifier(), lib.path()))
            .collect::<HashMap<xcframework::Identifier, PathBuf, RandomState>>();

        let id = xcframework::Identifier::new(
            target_arch.into(),
            target_os.into(),
            xcframework::Variant::Default,
        );

        let lib_path = native_libs.get(&id).expect("Library was not found");
        let lib_dir = lib_path.parent().unwrap();
        project_dir.push(lib_dir);

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
