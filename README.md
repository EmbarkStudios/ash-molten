<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🌋 ash-molten`

**Statically link with [MoltenVK]**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/ash-molten.svg)](https://crates.io/crates/ash-molten)
[![Docs](https://docs.rs/ash-molten/badge.svg)](https://docs.rs/ash-molten)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/ash-molten/status.svg)](https://deps.rs/repo/github/EmbarkStudios/ash-molten)
[![Build status](https://github.com/EmbarkStudios/ash-molten/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/ash-molten/actions)

</div>

`ash-molten` is built on top of [ash] and exposes a new entry point to statically link with [MoltenVK].

Requires Xcode 14 and Mac OS 10.15 (Catalina) to compile.

## Why?

* You want to compile down to a single binary that doesn't need any environment variables to bet set.
* You just want to try out [MoltenVK] without needing to setup the SDK.

## Why not?

* [ash] already supports [MoltenVK] via runtime linking. Runtime linking is the preferred way of using Vulkan because the loader can be updated at anytime without needing to recompile.
* `ash-molten` doesn't have access to the validation layers and therefore can not output any debug information.

## How?

```rust
let entry = ash_molten::MoltenEntry::load().expect("Unable to load Molten");
let app_name = CString::new("Hello Static Molten").unwrap();

let appinfo = vk::ApplicationInfo::builder()
    .application_name(&app_name)
    .application_version(0)
    .engine_name(&app_name)
    .engine_version(0)
    .api_version(vk_make_version!(1, 0, 0));

let create_info = vk::InstanceCreateInfo::builder().application_info(&appinfo);
let instance = entry.create_instance(&create_info, None).expect("Instance");
let devices = instance.enumerate_physical_devices();
println!("{:?}", devices);
```

You can run the example with `cargo run`.

## How does it work?

`ash-molten` links statically with [MoltenVK], it then uses `vkGetInstanceProcAddr` to resolve all the function pointers at runtime.

### Features

`cargo build` will clone a specific release of [MoltenVK] compile and statically link it with your application.
`cargo build --features pre-built` will download a pre-built version of MoltenVK from a release of ash-molten.
`cargo build --features external` provide own MoltenVK library.

If you want to compile [MoltenVK] yourself, you can use the `external` feature. `cargo build --features external` requires `libMoltenVK` to be visible (`LD_LIBRARY_PATH`).

### How to update

To update the version of [MoltenVK] uses, change the following:

* In `build.rs`, change `static VERSION = "1.1.0"` to the new [MoltenVK release](https://github.com/KhronosGroup/MoltenVK/releases) tag name
* Update the crate version in `Cargo.toml`
  * Bump the patch version
  * Set the version metadata to the MoltenVK release.
  * E.g. `0.2.0+1.1.9` -> `0.2.1+1.1.10`.
* Before you can submit the PR, you must also update the prebuilt version. See the next section.

### Updating pre-built version

To update the prebuilt version of MoltenVK that ash-molten uses, change the following:

* Follow the steps mentioned above.
* Download the MoltenVK XCFramework from, for example, the Vulkan SDK for Mac or build [MoltenVK] yourself.
  * in the case of downloading it from an external source make sure MoltenVK version matches `static VERSION`.
* From the XCFramework folder, from the built version of MoltenVK, zip the folders of platforms that need to be supported individually.
* Create a release with the tag: MoltenVK-{version number}.
* Upload the zip files to the release with the MoltenVK-{version number} tag.

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[MoltenVK]: https://github.com/KhronosGroup/MoltenVK
[ash]: https://github.com/MaikKlein/ash
