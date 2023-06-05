#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/SILOptions.hxx");

        // #[namespace = "cxx_swift::swift::ast::sil_options"]
        // #[cxx_name = "SILOptions"]
        // type SilOptions = crate::ffi::swift::ast::sil_options::SilOptions;
    }

    #[namespace = "cxx_swift::swift::ast::sil_options"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
