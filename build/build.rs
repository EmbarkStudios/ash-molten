mod xcframework;

mod mac {
    use std::path::{Path, PathBuf};

    // MoltenVK git tagged release to use
    pub static MOLTEN_VK_VERSION: &str = "1.1.10";
    pub static MOLTEN_VK_PATCH: Option<&str> = None;

    // The next two are useful for different kinds of bisection to find bugs.
    // MOLTEN_VK_LOCAL_BIN lets you specify a local MoltenVK binary directly from a Vulkan SDK, for example.
    // MOLTEN_VK_LOCAL lets you build directly from a local MoltenVK checkout, in which you can run
    // a `git bisect`, for example.
    // TODO: Make it possible to set these by environment variable?
    pub static MOLTEN_VK_LOCAL_BIN: Option<&str> = None; // for example, Some("/Users/my_user_name/VulkanSDK/1.3.211.0/MoltenVK")
    pub static MOLTEN_VK_LOCAL: Option<&str> = None; // for example, Some("/Users/my_user_name/dev/MoltenVK");

    // Return the artifact tag in the form of "x.x.x" or if there is a patch specified "x.x.x#yyyyyyy"
    pub(crate) fn get_artifact_tag() -> String {
        if let Some(patch) = MOLTEN_VK_PATCH {
            format!("{MOLTEN_VK_VERSION}#{patch}")
        } else {
            MOLTEN_VK_VERSION.to_owned()
        }
    }

    // Features are not used inside build scripts, so we have to explicitly query them from the
    // environment
    pub(crate) fn is_feature_enabled(feature: &str) -> bool {
        std::env::vars()
            .filter_map(|(flag, _)| {
                const NAME: &str = "CARGO_FEATURE_";
                if let Some(feat) = flag.strip_prefix(NAME) {
                    println!("{feat:?}");
                    Some(feat.to_owned())
                } else {
                    None
                }
            })
            .any(|f| f == feature)
    }

    pub(crate) fn build_molten() -> &'static str {
        use std::{
            process::Command,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
        };

        let checkout_dir = if let Some(local_dir) = MOLTEN_VK_LOCAL {
            PathBuf::from(local_dir)
        } else {
            Path::new(&std::env::var("OUT_DIR").expect("Couldn't find OUT_DIR"))
                .join(format!("MoltenVK-{}", get_artifact_tag()))
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

        if Path::new(&checkout_dir).exists() {
            // Don't pull if a specific hash has been checked out
            if MOLTEN_VK_PATCH.is_none() && MOLTEN_VK_LOCAL.is_none() {
                let git_status = Command::new("git")
                    .current_dir(&checkout_dir)
                    .arg("pull")
                    .status()
                    .expect("failed to spawn git");

                assert!(git_status.success(), "failed to pull MoltenVK from git");
            }
        } else if MOLTEN_VK_LOCAL.is_none() {
            let branch = format!("v{MOLTEN_VK_VERSION}");
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
                .status()
                .expect("failed to spawn git");

            assert!(git_status.success(), "failed to clone MoltenVK");
        }

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
                target => panic!("unknown target '{target}'"),
            },
            Err(e) => panic!("failed to determine target os '{e}'"),
        };

        let status = Command::new("sh")
            .current_dir(&checkout_dir)
            .arg("fetchDependencies")
            .arg(format!("--{target_name}"))
            .status()
            .expect("failed to spawn fetchDependencies");

        assert!(status.success(), "failed to fetchDependencies");

        println!("running make in {checkout_dir:?}");

        let status = Command::new("make")
            .current_dir(&checkout_dir)
            .arg(target_name)
            .status()
            .expect("failed to run make");

        assert!(status.success(), "failed to build MoltenVK");

        exit.store(true, Ordering::Release);
        handle.join().unwrap();
        target_name
    }

    pub(crate) fn download_prebuilt_molten<P: AsRef<Path>>(target_dir: &P) {
        use std::process::Command;

        std::fs::create_dir_all(target_dir).expect("Couldn't create directory");

        let download_url = format!(
            "https://github.com/EmbarkStudios/ash-molten/releases/download/MoltenVK-{}/MoltenVK.xcframework.zip",
            get_artifact_tag().replace('#', "%23")
        );
        let download_path = target_dir.as_ref().join("MoltenVK.xcframework.zip");

        let curl_status = Command::new("curl")
            .args(["--fail", "--location", "--silent", &download_url, "-o"])
            .arg(&download_path)
            .status()
            .expect("Couldn't launch curl");

        assert!(
            curl_status.success(),
            "failed to download prebuilt libraries"
        );

        let unzip_status = Command::new("unzip")
            .arg("-o")
            .arg(&download_path)
            .arg("-x")
            .arg("__MACOSX/*")
            .arg("-d")
            .arg(target_dir.as_ref())
            .status()
            .expect("Couldn't launch unzip");

        assert!(unzip_status.success(), "failed to run unzip");
    }
}

use std::{
    collections::{hash_map::RandomState, HashMap},
    path::{Path, PathBuf},
};

fn main() {
    use crate::mac::*;

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os != "macos" && target_os != "ios" {
        eprintln!("ash-molten requires either 'macos' or 'ios' target");
        return;
    }

    // The 'external' feature was not enabled. Molten will be built automatically.
    let external_enabled = is_feature_enabled("EXTERNAL");
    let pre_built_enabled = is_feature_enabled("PRE_BUILT") && MOLTEN_VK_LOCAL.is_none();

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    assert!(
        !(external_enabled && pre_built_enabled),
        "external and prebuilt cannot be active at the same time"
    );

    if !external_enabled {
        let mut project_dir = if let Some(local_bin_path) = MOLTEN_VK_LOCAL_BIN {
            let mut pb = PathBuf::from(local_bin_path);
            pb.push("MoltenVK.xcframework");
            pb
        } else if pre_built_enabled {
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
            let _target_name = build_molten();
            println!("Target dir was {target_dir:?}");

            let mut pb = PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").expect("unable to find env:CARGO_MANIFEST_DIR"),
            );
            if let Some(local) = MOLTEN_VK_LOCAL {
                pb.push(local);
            } else {
                pb.push(target_dir);
            }
            pb.push("Package/Latest/MoltenVK/MoltenVK.xcframework");

            pb
        };

        let xcframework = xcframework::XcFramework::parse(&project_dir)
            .unwrap_or_else(|_| panic!("Failed to parse XCFramework from {project_dir:?}"));
        let native_libs = xcframework
            .AvailableLibraries
            .into_iter()
            .flat_map(|lib| {
                lib.universal_to_native(project_dir.clone())
                    .expect("Failed to get native library")
            })
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
