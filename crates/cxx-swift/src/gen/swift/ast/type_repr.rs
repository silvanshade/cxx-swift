#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/TypeRepr.hxx");

        // #[namespace = "cxx_swift::swift::ast::type_repr"]
        // type TypeRepr<'ctx> = crate::ffi::swift::ast::type_repr::TypeRepr<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::type_repr"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
