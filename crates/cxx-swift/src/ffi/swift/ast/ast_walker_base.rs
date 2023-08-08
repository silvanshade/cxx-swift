use crate::{ffi::swift::ast::ast_walker::AstWalker, gen::swift::ast::ast_walker_base};
use core::pin::Pin;

pub use crate::auto::swift::ast::ast_walker_base::AstWalkerBase;

impl<'ctx> core::ops::Deref for AstWalkerBase<'ctx> {
    type Target = AstWalker<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        ast_walker_base::deref(self)
    }
}

impl<'ctx> AstWalkerBase<'ctx> {
    #[inline]
    pub fn deref_pin(self: Pin<&mut Self>) -> Pin<&mut AstWalker<'ctx>> {
        ast_walker_base::deref_pin(self)
    }
}

impl<'ctx> AstWalkerBase<'ctx> {
    #[inline]
    pub unsafe fn new() -> impl moveref::New<Output = AstWalkerBase<'ctx>> {
        Self::default_new()
    }
}

impl<'ctx> AstWalkerBase<'ctx> {
    #[inline]
    pub unsafe fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_into_generic_params(self)
    }

    #[inline]
    pub unsafe fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_into_tap_expression(self)
    }

    #[inline]
    pub unsafe fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_into_property_wrapper_placeholder_value(self)
    }

    #[inline]
    pub unsafe fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_capture_initializer_expressions(self)
    }

    #[inline]
    pub unsafe fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_accessors_the_old_way(self)
    }

    #[inline]
    pub unsafe fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        ast_walker_base::should_walk_serialized_top_level_internal_decls(self)
    }
}
