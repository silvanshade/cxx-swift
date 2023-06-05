#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/PreWalkResultExpr.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_walker::pre_walk_result_expr"]
        type PreWalkResultExpr<'ctx> =
            crate::ffi::swift::ast::ast_walker::pre_walk_result_expr::PreWalkResultExpr<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker::pre_walk_result_expr"]
    unsafe extern "C++" {}
}
pub(crate) use self::ffi::*;
