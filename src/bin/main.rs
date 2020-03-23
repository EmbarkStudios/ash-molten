use ash::{
    version::{EntryV1_0, InstanceV1_0},
    vk,
};
use std::ffi::CString;
fn main() {
    unsafe {
        let entry = ash_molten::MoltenEntry::load().expect("Unable to load Molten");
        let app_name = CString::new("Hello Static Molten").unwrap();

        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk::make_version(1, 0, 0));

        let create_info = vk::InstanceCreateInfo::builder().application_info(&appinfo);
        let instance = entry.create_instance(&create_info, None).expect("Instance");
        let devices = instance.enumerate_physical_devices();
        println!("{:?}", devices);
    }
}
