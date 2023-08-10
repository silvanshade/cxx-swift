pub(crate) mod parent_ty;
pub(crate) mod pre_walk_result_expr;
pub(crate) mod vtable;

use crate::{ffi::swift::ast::ast_walker::parent_ty::ParentTy, gen::swift::ast::ast_walker};
use core::pin::Pin;

pub use crate::auto::swift::ast::ast_walker::AstWalker;

impl<'ctx> AstWalker<'ctx> {
    #[inline]
    pub fn get_parent(&self) -> ParentTy<'ctx> {
        ast_walker::get_parent(self)
    }

    #[inline]
    pub fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_into_generic_params(self)
    }

    #[inline]
    pub fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_into_tap_expression(self)
    }

    #[inline]
    pub fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_into_property_wrapper_placeholder_value(self)
    }

    #[inline]
    pub fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_capture_initializer_expressions(self)
    }

    #[inline]
    pub fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_accessors_the_old_way(self)
    }

    #[inline]
    pub fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        ast_walker::should_walk_serialized_top_level_internal_decls(self)
    }
}
