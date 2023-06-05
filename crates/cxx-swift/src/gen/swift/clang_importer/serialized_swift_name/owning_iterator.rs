#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/OwningIterator.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
        type SerializedSwiftName<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::owning_iterator"]
        type SerializedSwiftNameOwningIterator<'ctx> = crate::ffi::swift::clang_importer::serialized_swift_name::owning_iterator::SerializedSwiftNameOwningIterator<'ctx>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::owning_iterator"]
    unsafe extern "C++" {
        fn next<'ctx>(This: Pin<&mut SerializedSwiftNameOwningIterator<'ctx>>) -> *mut SerializedSwiftName<'ctx>;
    }
}
pub(crate) use self::ffi::*;
