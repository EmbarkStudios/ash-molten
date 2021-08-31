#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize)]
#[serde(into = "&'static str")]
#[serde(from = "String")]
pub enum Arch {
    X86,
    Amd64,
    Arm64,
    Arm64e,
    Unknown,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize)]
#[serde(into = "&'static str")]
#[serde(from = "String")]
pub enum Platform {
    MacOs,
    Ios,
    TvOs,
    WatchOs,
    Unknown,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize)]
#[serde(into = "&'static str")]
#[serde(from = "String")]
pub enum Variant {
    Default,
    Simulator,
}

impl<T: AsRef<str>> From<T> for Arch {
    fn from(arch: T) -> Self {
        match arch.as_ref() {
            "x86_64" => Arch::Amd64,
            "x86" => Arch::X86,
            "arm64" | "aarch64" => Arch::Arm64,
            "arm64e" => Arch::Arm64e,
            _ => Arch::Unknown,
        }
    }
}

impl<'a> From<Arch> for &'a str {
    fn from(arch: Arch) -> Self {
        match arch {
            Arch::Amd64 => "x86_64",
            Arch::X86 => "x86",
            Arch::Arm64 => "arm64",
            Arch::Arm64e => "arm64e",
            Arch::Unknown => "",
        }
    }
}

impl<T: AsRef<str>> From<T> for Platform {
    fn from(platform: T) -> Self {
        match platform.as_ref() {
            "tvos" => Platform::TvOs,
            "macos" => Platform::MacOs,
            "ios" => Platform::Ios,
            "watchos" => Platform::WatchOs,
            _ => Platform::Unknown,
        }
    }
}

impl<'a> From<Platform> for &'a str {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::TvOs => "tvos",
            Platform::MacOs => "macos",
            Platform::Ios => "ios",
            Platform::WatchOs => "watchos",
            Platform::Unknown => "",
        }
    }
}

impl<T: AsRef<str>> From<T> for Variant {
    fn from(variant: T) -> Self {
        match variant.as_ref() {
            "simulator" => Variant::Simulator,
            _ => Variant::Default,
        }
    }
}

impl<'a> From<Variant> for &'a str {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Simulator => "simulator",
            Variant::Default => "",
        }
    }
}
