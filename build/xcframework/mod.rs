use anyhow::Error;
use std::{fs::File, io::BufReader, path::Path};

mod common;
mod library;

pub use common::*;
pub use library::*;

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub struct XcFramework {
    pub AvailableLibraries: Vec<library::UniversalLibrary>,
    pub CFBundlePackageType: String,
    pub XCFrameworkFormatVersion: String,
}

impl XcFramework {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<XcFramework, Error> {
        let mut reader = BufReader::new(File::open(path.as_ref().join("Info.plist"))?);
        Ok(plist::from_reader(&mut reader)?)
    }
}
