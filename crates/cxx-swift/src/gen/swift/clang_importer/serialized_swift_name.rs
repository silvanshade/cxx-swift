pub(crate) mod owning_iterator;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/StringRef.hxx");

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
