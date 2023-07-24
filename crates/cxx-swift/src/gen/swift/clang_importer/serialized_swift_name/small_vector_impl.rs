#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVectorImpl.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
        type SerializedSwiftName<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::small_vector_impl::SmallVectorImpl<'ctx>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl"]
    unsafe extern "C++" {
        fn as_slice<'this, 'ctx>(This: &'this SmallVectorImpl<'ctx>) -> &'this [SerializedSwiftName<'ctx>];

        unsafe fn as_mut_slice<'this, 'ctx>(
            This: Pin<&'this mut SmallVectorImpl<'ctx>>,
        ) -> &'this mut [SerializedSwiftName<'ctx>];
    }
}
pub(crate) use self::ffi::*;
