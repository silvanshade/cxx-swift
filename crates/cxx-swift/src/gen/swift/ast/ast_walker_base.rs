#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalkerBase.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_walker"]
        #[cxx_name = "ASTWalker"]
        type AstWalker<'ctx> = crate::ffi::swift::ast::ast_walker::AstWalker<'ctx>;

        #[namespace = "cxx_swift::swift::ast::ast_walker_base"]
        #[cxx_name = "ASTWalkerBase"]
        type AstWalkerBase<'ctx> = crate::ffi::swift::ast::ast_walker_base::AstWalkerBase<'ctx>;
    }

    // #[namespace = "cxx_swift::swift::ast_walker_base"]
    // unsafe extern "C++" {
    //     unsafe fn cxx_placement_new<'ctx>(This: *mut AstWalkerBase<'ctx>);
    // }

    #[namespace = "cxx_swift::swift::ast::ast_walker_base"]
    unsafe extern "C++" {
        fn deref<'this, 'ctx>(This: &'this AstWalkerBase<'ctx>) -> &'this AstWalker<'ctx>;

        fn deref_pin<'this, 'ctx>(This: Pin<&'this mut AstWalkerBase<'ctx>>) -> Pin<&'this mut AstWalker<'ctx>>;
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker_base"]
    unsafe extern "C++" {
        unsafe fn should_walk_into_generic_params<'ctx>(This: Pin<&mut AstWalkerBase<'ctx>>) -> bool;

        unsafe fn should_walk_into_tap_expression<'ctx>(This: Pin<&mut AstWalkerBase<'ctx>>) -> bool;

        unsafe fn should_walk_into_property_wrapper_placeholder_value<'ctx>(
            This: Pin<&mut AstWalkerBase<'ctx>>,
        ) -> bool;

        unsafe fn should_walk_capture_initializer_expressions<'ctx>(This: Pin<&mut AstWalkerBase<'ctx>>) -> bool;

        unsafe fn should_walk_accessors_the_old_way<'ctx>(This: Pin<&mut AstWalkerBase<'ctx>>) -> bool;

        unsafe fn should_walk_serialized_top_level_internal_decls<'ctx>(This: Pin<&mut AstWalkerBase<'ctx>>) -> bool;
    }
}
pub(crate) use self::ffi::*;
