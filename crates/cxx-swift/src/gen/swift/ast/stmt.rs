#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Stmt.hxx");

        // #[namespace = "cxx_swift::swift::ast::stmt"]
        // type Stmt<'ctx> = crate::ffi::swift::ast::stmt::Stmt<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::stmt"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
