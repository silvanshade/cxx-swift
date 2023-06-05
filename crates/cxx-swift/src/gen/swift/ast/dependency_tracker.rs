#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/DependencyTracker.hxx");

        // #[namespace = "cxx_swift::swift::ast::dependency_tracker"]
        // type DependencyTracker = crate::ffi::swift::ast::dependency_tracker::DependencyTracker;
    }

    #[namespace = "cxx_swift::swift::ast::dependency_tracker"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
