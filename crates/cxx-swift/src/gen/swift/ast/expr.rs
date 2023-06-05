#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Expr.hxx");

        // #[namespace = "cxx_swift::swift::ast::expr"]
        // type Expr<'ctx> = crate::ffi::swift::ast::expr::Expr<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::expr"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
