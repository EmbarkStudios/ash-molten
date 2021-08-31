// BEGIN - Embark standard lints v0.4
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.4
// crate-specific exceptions:
#![allow(unsafe_code)]

mod xcframework;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac {

    use std::path::Path;

    // MoltenVK git tagged release to use
    pub static MOLTEN_VK_VERSION: &str = "1.1.5";
    pub static MOLTEN_VK_PATCH: Option<&str> = None;

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
        use std::process::Command;

        std::fs::create_dir_all(&target_dir).expect("Couldn't create directory");

        let download_url = format!(
            "https://github.com/EmbarkStudios/ash-molten/releases/download/MoltenVK-{}/MoltenVK.xcframework.zip",
            get_artifact_tag().replace("#", "%23")
        );
        let download_path = target_dir.as_ref().join("MoltenVK.xcframework.zip");

        let curl_status = Command::new("curl")
            .args(&["--location", "--silent", &download_url, "-o"])
            .arg(&download_path)
            .spawn()
            .expect("Couldn't launch curl")
            .wait()
            .expect("failed to wait on curl");

        assert!(curl_status.success());

        let unzip_status = Command::new("unzip")
            .arg("-o")
            .arg(&download_path)
            .arg("-x")
            .arg("__MACOSX/*")
            .arg("-d")
            .arg(target_dir.as_ref())
            .spawn()
            .expect("Couldn't launch unzip")
            .wait()
            .expect("failed to wait on unzip");

        if !unzip_status.success() {
            let bytes = std::fs::read(download_path)
                .expect("unzip failed, and further, could not open output zip file");
            match std::str::from_utf8(&bytes) {
                Ok(string) => {
                    panic!(
                        "Could not unzip MoltenVK.xcframework.zip. File was utf8, perhaps an error?\n{}",
                        string
                    );
                }
                Err(_) => {
                    panic!("Could not unzip MoltenVK.xcframework.zip");
                }
            }
        }
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

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    eprintln!("ash-molten requires either 'macos' or 'ios' target");
}
