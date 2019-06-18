# ash-molten
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

`ash-molten` ships with a prebuilt library which you can find in [`external`](external/).

[MoltenVK](https://github.com/KhronosGroup/MoltenVK) is built via `build_molten.sh`. [MoltenVK](https://github.com/KhronosGroup/MoltenVK) is added as a git submodule. See the commit hash to find out which version `ash-molten` uses.

### Features

`cargo build` will automatically compile molten for you. If you want to compile molten yourself, you can use the `external` feature. `cargo build --features external` requires libMoltenVK to be visible. You have to manually set `LD_LIBRARY_PATH`.

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
