// BEGIN - Embark standard lints v0.4
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.4
// crate-specific exceptions:
#![allow(unsafe_code)]

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
            get_instance_proc_addr,
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
