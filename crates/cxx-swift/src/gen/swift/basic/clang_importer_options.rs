#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-auto/cxx/include/cxx-auto.hxx");
        include!("cxx-swift-auto/cxx/include/swift/Basic/ClangImporterOptions.hxx");

        #[cxx_name = "c_char"]
        type _c_char = cxx_auto::ctypes::c_char;

        #[namespace = "cxx_swift::swift::basic::clang_importer_options"]
        type ClangImporterOptions = crate::ffi::swift::basic::clang_importer_options::ClangImporterOptions;
    }

    #[namespace = "cxx_swift::swift::basic::clang_importer_options"]
    unsafe extern "C++" {
        fn modify_extra_args_push_back(This: Pin<&mut ClangImporterOptions>, slice: &[_c_char]);

        fn set_bridging_header(This: Pin<&mut ClangImporterOptions>, slice: &[_c_char]);

        fn set_module_cache_path(This: Pin<&mut ClangImporterOptions>, slice: &[_c_char]);

        fn set_precompiled_header_output_dir(This: Pin<&mut ClangImporterOptions>, slice: &[_c_char]);
    }
}
pub(crate) use self::ffi::*;
