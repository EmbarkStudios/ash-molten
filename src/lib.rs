use ash::{vk, Entry};

extern "system" {
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

/// Fetches the function pointer to `vkGetInstanceProcAddr` which is statically linked.
pub fn load() -> Entry {
    let static_fn = vk::StaticFn {
        get_instance_proc_addr: vkGetInstanceProcAddr,
    };
    #[allow(unsafe_code)]
    unsafe {
        Entry::from_static_fn(static_fn)
    }
}
