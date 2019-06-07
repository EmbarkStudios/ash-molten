# ash-molten


`ash-molten` statically links with molten and exposes a new entry point `MoltenEntry`. The function pointers are still fetched at runtime via `getInstanceProcAddr`.

Use this if want to compile down to a single executable. Runtime linking is always preferred and you lose access to the validation layers if you use `ash-molten`.

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
