use crate::ffi::swift::ast::ast_walker::vtable::AstWalkerDyn;

unsafe impl<'state, 'ctx> cxx::ExternType for AstWalkerRust<'state, 'ctx> {
    type Id = cxx::type_id!("cxx_swift::swift::ast::ast_walker_rust::ASTWalkerRust");
    type Kind = cxx::kind::Opaque;
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        #[cxx_name = "ASTWalkerDyn"]
        type AstWalkerDyn<'state, 'ctx>;

        #[cxx_name = "shouldWalkIntoGenericParams"]
        fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool;

        #[cxx_name = "shouldWalkIntoTapExpression"]
        fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool;

        #[cxx_name = "shouldWalkIntoPropertyWrapperPlaceholderValue"]
        fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool;

        #[cxx_name = "shouldWalkCaptureInitializerExpressions"]
        fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool;

        #[cxx_name = "shouldWalkAccessorsTheOldWay"]
        fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool;

        #[cxx_name = "shouldWalkSerializedTopLevelInternalDecls"]
        fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool;
    }

    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker.hxx");
        include!("cxx-swift/cxx/include/swift/AST/ASTWalkerRust.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_walker"]
        #[cxx_name = "ASTWalker"]
        type AstWalker<'ctx> = crate::ffi::swift::ast::ast_walker::AstWalker<'ctx>;

        #[namespace = "cxx_swift::swift::ast::ast_walker_rust"]
        #[cxx_name = "ASTWalkerRust"]
        type AstWalkerRust<'state, 'ctx> = crate::ffi::swift::ast::ast_walker_rust::AstWalkerRust<'state, 'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker_rust"]
    unsafe extern "C++" {
        unsafe fn cxx_placement_new<'state, 'ctx>(
            This: *mut AstWalkerRust<'state, 'ctx>,
            ast_walker_dyn: Box<AstWalkerDyn<'state, 'ctx>>,
        );
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker_rust"]
    unsafe extern "C++" {
        fn deref<'this, 'state, 'ctx>(This: &'this AstWalkerRust<'state, 'ctx>) -> &'this AstWalker<'ctx>;

        fn deref_pin<'this, 'state, 'ctx>(
            This: Pin<&'this mut AstWalkerRust<'state, 'ctx>>,
        ) -> Pin<&'this mut AstWalker<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
