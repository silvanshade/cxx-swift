#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/DiagnosticEngine.hxx");
        include!("cxx-swift-abi/cxx/include/swift/Basic/SourceManager.hxx");

        #[namespace = "cxx_swift::swift::ast::diagnostic_engine"]
        type DiagnosticEngine<'source_manager> =
            crate::ffi::swift::ast::diagnostic_engine::DiagnosticEngine<'source_manager>;

        #[namespace = "cxx_swift::swift::basic::source_manager"]
        type SourceManager = crate::ffi::swift::basic::source_manager::SourceManager;
    }

    #[namespace = "cxx_swift::swift::ast::diagnostic_engine"]
    unsafe extern "C++" {
        unsafe fn cxx_placement_new_from_source_manager(
            This: *mut DiagnosticEngine,
            source_manager: Pin<&mut SourceManager>,
        );
    }
}
pub(crate) use self::ffi::*;
