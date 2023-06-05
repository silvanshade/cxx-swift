#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/Basic/SourceManager.hxx");

        // #[namespace = "cxx_swift::swift::basic::source_manager"]
        // type SourceManager = crate::ffi::swift::basic::source_manager::SourceManager;
    }

    #[namespace = "cxx_swift::swift::basic::source_manager"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
