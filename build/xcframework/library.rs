use anyhow::Error;

use super::common::{Arch, Platform, Variant};
use std::{
    path::{Path, PathBuf},
    process::Command,
    string::String,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Identifier {
    pub arch: Arch,
    pub platform: Platform,
    pub variant: Variant,
}

impl Identifier {
    pub fn new(arch: Arch, platform: Platform, variant: Variant) -> Self {
        Self {
            arch,
            platform,
            variant,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub struct UniversalLibrary {
    LibraryPath: String,
    SupportedArchitectures: Vec<Arch>,
    SupportedPlatformVariant: Option<Variant>,
    SupportedPlatform: Platform,
    LibraryIdentifier: String,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct NativeLibrary {
    LibraryPath: String,
    SupportedArchitectures: Arch,
    SupportedPlatformVariant: Option<Variant>,
    SupportedPlatform: Platform,
    LibraryIdentifier: String,
}

impl UniversalLibrary {
    pub fn universal_to_native<P: AsRef<Path>>(
        self,
        xcframework_dir: P,
    ) -> Result<Vec<NativeLibrary>, Error> {
        let lib_id = &self.LibraryIdentifier;
        let platform = &self.SupportedPlatform;
        let variant = self.SupportedPlatformVariant.as_ref();
        let lib_path = &self.LibraryPath;

        if self.SupportedArchitectures.len() == 1 {
            let arch = &self.SupportedArchitectures[0];
            Ok(vec![NativeLibrary {
                LibraryPath: lib_path.into(),
                SupportedArchitectures: *arch,
                SupportedPlatformVariant: variant.cloned(),
                SupportedPlatform: *platform,
                LibraryIdentifier: lib_id.into(),
            }])
        } else {
            let mut native_libs = Vec::new();
            let xcframework_dir = xcframework_dir.as_ref().to_path_buf();
            let full_path = xcframework_dir.join(lib_id).join(lib_path);

            for arch in &self.SupportedArchitectures {
                let platform_str: &str = (*platform).into();
                let arch_str: &str = (*arch).into();
                let mut new_identifier = format!("{}-{}", platform_str, arch_str);

                if let Some(variant) = variant {
                    let variant_str: &str = (*variant).into();
                    new_identifier.push_str(format!("-{}", variant_str).as_str());
                }

                let mut out_path = xcframework_dir.join(new_identifier.clone());
                std::fs::create_dir_all(&out_path)?;
                out_path.push(lib_path);

                assert!(Command::new("lipo")
                    .arg(&full_path)
                    .arg("-thin")
                    .arg(arch_str)
                    .arg("-output")
                    .arg(out_path)
                    .status()
                    .expect("Failed to spawn lipo")
                    .success());

                native_libs.push(NativeLibrary {
                    LibraryPath: lib_path.into(),
                    SupportedArchitectures: *arch,
                    SupportedPlatformVariant: variant.copied(),
                    SupportedPlatform: *platform,
                    LibraryIdentifier: new_identifier,
                });
            }

            Ok(native_libs)
        }
    }
}

impl NativeLibrary {
    pub fn path(&self) -> PathBuf {
        Path::new(&format!("{}/{}", self.LibraryIdentifier, self.LibraryPath)).to_path_buf()
    }

    pub fn identifier(&self) -> Identifier {
        Identifier::new(
            self.SupportedArchitectures,
            self.SupportedPlatform,
            self.SupportedPlatformVariant.unwrap_or(Variant::Default),
        )
    }
}
