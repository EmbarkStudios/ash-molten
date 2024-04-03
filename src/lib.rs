use ash::vk;

extern "system" {
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

/// Fetches the function pointer to `vkGetInstanceProcAddr` which is statically linked.
pub fn load() -> ash::Entry {
    let static_fn = ash::StaticFn {
        get_instance_proc_addr: vkGetInstanceProcAddr,
    };
    #[allow(unsafe_code)]
    unsafe {
        ash::Entry::from_static_fn(static_fn)
    }
}
