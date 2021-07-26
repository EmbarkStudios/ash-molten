// BEGIN - Embark standard lints v0.3
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::explicit_into_iter_loop,
    clippy::filter_map_next,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::verbose_file_reads,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.3
// crate-specific exceptions:
#![allow(unsafe_code)]

use ash::{vk, EntryCustom, LoadingError};
use std::ops::{Deref, DerefMut};

extern "system" {
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

/// ZST used as a tag for [ash::EntryCustom]
pub struct MoltenLibStatic;

/// The entry point for the statically linked molten library
pub struct MoltenEntry(EntryCustom<MoltenLibStatic>);

impl MoltenEntry {
    /// Fetches the function pointer to `vkGetInstanceProcAddr` which is statically linked. This
    /// function can not fail.
    pub fn load() -> Result<MoltenEntry, LoadingError> {
        // Defer the rest of the loading to EntryCustom
        Ok(MoltenEntry(EntryCustom::new_custom(
            MoltenLibStatic,
            Self::static_loader,
        )))
    }
    #[doc(hidden)]
    fn static_loader(
        _lib: &mut MoltenLibStatic,
        _name: &std::ffi::CStr,
    ) -> *const core::ffi::c_void {
        // Cast function pointer to *const c_void; EntryCustom::new_custom calls StaticFn::load
        // which performs the reverse operation using std::mem::transmute.
        vkGetInstanceProcAddr as *const core::ffi::c_void
    }
}
impl Deref for MoltenEntry {
    type Target = EntryCustom<MoltenLibStatic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MoltenEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
