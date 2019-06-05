extern crate ash;
use ash::{version::EntryV1_0, vk, Instance, InstanceError, RawPtr};

extern "system" {
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    p_name: *const std::os::raw::c_char,
) -> vk::PFN_vkVoidFunction {
    unsafe { vkGetInstanceProcAddr(instance, p_name) }
}

/// The entry point for the statically linked molten library
pub struct MoltenEntry {
    static_fn: vk::StaticFn,
    entry_fn_1_0: vk::EntryFnV1_0,
}

impl MoltenEntry {
    /// Fetches the function pointer to `get_instance_proc_addr` which is statically linked. This
    /// function can not fail.
    pub fn load() -> Result<MoltenEntry, ash::LoadingError> {
        let static_fn = vk::StaticFn {
            get_instance_proc_addr: get_instance_proc_addr,
        };

        let entry_fn_1_0 = vk::EntryFnV1_0::load(|name| unsafe {
            std::mem::transmute(
                static_fn.get_instance_proc_addr(vk::Instance::null(), name.as_ptr()),
            )
        });

        Ok(MoltenEntry {
            static_fn,
            entry_fn_1_0,
        })
    }
}
impl EntryV1_0 for MoltenEntry {
    type Instance = Instance;
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateInstance.html>"]
    unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, InstanceError> {
        let mut instance: vk::Instance = vk::Instance::null();
        let err_code = self.fp_v1_0().create_instance(
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut instance,
        );
        if err_code != vk::Result::SUCCESS {
            return Err(InstanceError::VkError(err_code));
        }
        Ok(Instance::load(&self.static_fn, instance))
    }
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        &self.entry_fn_1_0
    }
    fn static_fn(&self) -> &vk::StaticFn {
        &self.static_fn
    }
}
