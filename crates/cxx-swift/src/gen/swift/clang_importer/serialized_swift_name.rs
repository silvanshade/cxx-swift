pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        include!("cxx-llvm-auto/cxx/include/llvm/ADT/StringRef.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
        type SerializedSwiftName<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName<'ctx>;

        #[namespace = "cxx_llvm::llvm::adt::string_ref"]
        type StringRef<'a> = cxx_llvm::llvm::adt::string_ref::StringRef<'a>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
    unsafe extern "C++" {
        fn get_name<'ctx>(This: &SerializedSwiftName<'ctx>) -> StringRef<'ctx>;
    }
}
pub(crate) use self::ffi::*;
