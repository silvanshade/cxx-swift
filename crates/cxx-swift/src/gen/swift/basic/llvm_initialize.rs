#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/Basic/LLVMInitialize.hxx");

        #[namespace = "cxx_swift::swift::basic::llvm_initialize"]
        fn initialize_llvm();
    }
}
pub use self::ffi::initialize_llvm;
