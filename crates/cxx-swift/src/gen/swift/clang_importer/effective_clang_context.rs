#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/EffectiveClangContext.hxx");

        // #[namespace = "cxx_swift::swift::clang_importer::effective_clang_context"]
        // type EffectiveClangContext = crate::ffi::swift::clang_importer::effective_clang_context::EffectiveClangContext;
    }

    #[namespace = "cxx_swift::swift::clang_importer::effective_clang_context"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
