use crate::gen::swift::basic::clang_importer_options;
use core::pin::Pin;

pub use crate::abi::swift::basic::clang_importer_options::ClangImporterOptions;

impl ClangImporterOptions {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = ClangImporterOptions> {
        Self::default_new()
    }
}

impl ClangImporterOptions {
    #[inline]
    pub fn modify_extra_args_push_back(self: Pin<&mut Self>, str: &str) {
        let bytes = str.as_bytes();
        let slice = cxx_memory_abi::ctypes::c_char::from_bytes(bytes);
        clang_importer_options::modify_extra_args_push_back(self, slice)
    }

    #[inline]
    pub fn set_bridging_header(self: Pin<&mut Self>, path: &std::path::Path) {
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        let slice = cxx_memory_abi::ctypes::c_char::from_bytes(bytes);
        clang_importer_options::set_bridging_header(self, slice);
    }

    #[inline]
    pub fn set_module_cache_path(self: Pin<&mut Self>, path: &std::path::Path) {
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        let slice = cxx_memory_abi::ctypes::c_char::from_bytes(bytes);
        clang_importer_options::set_module_cache_path(self, slice);
    }

    #[inline]
    pub fn set_precompiled_header_output_dir(self: Pin<&mut Self>, path: &std::path::Path) {
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        let slice = cxx_memory_abi::ctypes::c_char::from_bytes(bytes);
        clang_importer_options::set_precompiled_header_output_dir(self, slice);
    }
}
