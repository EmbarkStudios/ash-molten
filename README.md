# 🌋 ash-molten

[![Build Status](https://travis-ci.com/EmbarkStudios/ash-molten.svg?branch=master)](https://travis-ci.com/EmbarkStudios/ash-molten)
[![Latest version](https://img.shields.io/crates/v/ash-molten.svg)](https://crates.io/crates/ash-molten)
[![Docs](https://docs.rs/ash-molten/badge.svg)](https://docs.rs/ash-molten)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.games)

`ash-molten` is built on top of [ash](https://github.com/MaikKlein/ash) and exposes a new entry point to statically link with [MoltenVK](https://github.com/KhronosGroup/MoltenVK).

## Why?

* You want to compile down to a single binary that doesn't need any enviroment variables to bet set.

* You just want to try out [MoltenVK](https://github.com/KhronosGroup/MoltenVK) without needing to setup the SDK.

## Why not?

* [ash](https://github.com/MaikKlein/ash) already supports [MoltenVK](https://github.com/KhronosGroup/MoltenVK) via runtime linking. Runtime linking is the prefered way of using Vulkan because the loader can be updated at anytime without needing to recompile.

* `ash-molten` doesn't have access to the validation layers and thefore can not output any debug information.

## How?
```Rust
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

`ash-molten` links statically with [MoltenVK](https://github.com/KhronosGroup/MoltenVK), it then uses `vkGetInstanceProcAddr` to resolve all the function pointers at runtime.

### Features

`cargo build` will clone a specific release of [MoltenVK](https://github.com/KhronosGroup/MoltenVK) compile and statically link it with your application. 

If you want to compile [MoltenVK](https://github.com/KhronosGroup/MoltenVK) yourself, you can use the `external` feature. `cargo build --features external` requires `libMoltenVK` to be visible (`LD_LIBRARY_PATH`).

### How to update

To update the version of [MoltenVK](https://github.com/KhronosGroup/MoltenVK) uses, change the following:

- In `build.rs`, change `let tag = "v1.0.37"` to the new [MoltenVK release](https://github.com/KhronosGroup/MoltenVK/releases) tag name
- Update the crate version in `Cargo.toml`
  - Bump the patch version
  - Set the version metadata to the MoltenVK release. 
  - E.g. `0.2.0+37` -> `0.2.1+38`.

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
