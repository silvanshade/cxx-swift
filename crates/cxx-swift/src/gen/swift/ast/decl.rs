#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/AST/Decl.hxx");

        // #[namespace = "cxx_swift::swift::ast::decl"]
        // type Decl<'ctx> = crate::ffi::swift::ast::decl::Decl<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::decl"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
