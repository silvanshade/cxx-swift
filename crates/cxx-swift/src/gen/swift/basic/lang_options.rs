#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-llvm-auto/cxx/include/llvm/ADT/Triple.hxx");
        include!("cxx-swift-auto/cxx/include/swift/Basic/LangOptions.hxx");

        #[namespace = "cxx_swift::swift::basic::lang_options"]
        type LangOptions = crate::ffi::swift::basic::lang_options::LangOptions;

        #[namespace = "cxx_llvm::llvm::adt::triple"]
        type Triple = cxx_llvm::llvm::adt::triple::Triple;
    }

    #[namespace = "cxx_swift::swift::basic::lang_options"]
    unsafe extern "C++" {
        unsafe fn set_target(
            This: Pin<&mut LangOptions>,
            triple: *mut Triple,
            os_was_invalid: &mut bool,
            arch_was_invalid: &mut bool,
        );
    }
}
pub(crate) use self::ffi::*;
