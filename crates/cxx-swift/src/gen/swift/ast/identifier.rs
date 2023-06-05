#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Identifier.hxx");

        // #[namespace = "cxx_swift::swift::ast::identifier"]
        // type Identifier<'ctx> = crate::ffi::swift::ast::identifier::Identifier<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::identifier"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
