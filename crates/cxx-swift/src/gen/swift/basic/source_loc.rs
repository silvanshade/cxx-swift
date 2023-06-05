#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/Basic/SourceLoc.hxx");

        // #[namespace = "cxx_swift::swift::basic::source_loc"]
        // type SourceLoc = crate::ffi::swift::basic::source_loc::SourceLoc;
    }

    #[namespace = "cxx_swift::swift::basic::source_loc"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
