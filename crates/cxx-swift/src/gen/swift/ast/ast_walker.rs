#![allow(unused)] // FIXME

mod parent_ty;
mod pre_walk_result_expr;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/ParentTy.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/PreWalkResultExpr.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/Expr.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_walker"]
        #[cxx_name = "ASTWalker"]
        type AstWalker<'ctx> = crate::ffi::swift::ast::ast_walker::AstWalker<'ctx>;

        #[namespace = "cxx_swift::swift::ast::expr"]
        #[cxx_name = "Expr"]
        type Expr<'ctx> = crate::ffi::swift::ast::expr::Expr<'ctx>;

        #[namespace = "cxx_swift::swift::ast::ast_walker::parent_ty"]
        #[cxx_name = "ParentTy"]
        type ParentTy<'ctx> = crate::ffi::swift::ast::ast_walker::parent_ty::ParentTy<'ctx>;

        #[namespace = "cxx_swift::swift::ast::ast_walker::pre_walk_result_expr"]
        #[cxx_name = "PreWalkResultExpr"]
        type PreWalkResultExpr<'ctx> =
            crate::ffi::swift::ast::ast_walker::pre_walk_result_expr::PreWalkResultExpr<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker"]
    unsafe extern "C++" {
        unsafe fn get_parent<'ctx>(This: &AstWalker<'ctx>) -> ParentTy<'ctx>;

        unsafe fn walk_to_expr_pre<'ctx>(
            This: Pin<&mut AstWalker<'ctx>>,
            expr: *mut Expr<'ctx>,
            out: *mut PreWalkResultExpr,
        );

        unsafe fn should_walk_into_generic_params<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;

        unsafe fn should_walk_into_tap_expression<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;

        unsafe fn should_walk_into_property_wrapper_placeholder_value<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;

        unsafe fn should_walk_capture_initializer_expressions<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;

        unsafe fn should_walk_accessors_the_old_way<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;

        unsafe fn should_walk_serialized_top_level_internal_decls<'ctx>(This: Pin<&mut AstWalker<'ctx>>) -> bool;
    }
}
pub(crate) use self::ffi::*;
