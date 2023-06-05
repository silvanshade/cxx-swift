#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/ParentTy.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_walker::parent_ty"]
        type ParentTy<'ctx> = crate::ffi::swift::ast::ast_walker::parent_ty::ParentTy<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::ast_walker::parent_ty"]
    unsafe extern "C++" {}
}
pub(crate) use self::ffi::*;
