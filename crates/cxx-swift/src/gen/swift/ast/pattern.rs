#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Pattern.hxx");

        // #[namespace = "cxx_swift::swift::ast::pattern"]
        // type Pattern<'ctx> = crate::ffi::swift::ast::pattern::Pattern<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::pattern"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
