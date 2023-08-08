#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/Basic/TypeCheckerOptions.hxx");

        // #[namespace = "cxx_swift::swift::basic::type_checker_options"]
        // type TypeCheckerOptions = crate::ffi::swift::basic::type_checker_options::TypeCheckerOptions;
    }

    #[namespace = "cxx_swift::swift::basic::type_checker_options"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
