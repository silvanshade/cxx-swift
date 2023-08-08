#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVector.hxx");
        include!("cxx-swift-auto/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVectorImpl.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector"]
        type SmallVector<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::small_vector::SmallVector<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::small_vector_impl::SmallVectorImpl<'ctx>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector"]
    unsafe extern "C++" {
        fn as_ref_small_vector_impl<'this, 'ctx>(This: &'this SmallVector<'ctx>) -> &'this SmallVectorImpl<'ctx>;

        fn as_pin_small_vector_impl<'this, 'ctx>(
            This: Pin<&'this mut SmallVector<'ctx>>,
        ) -> Pin<&'this mut SmallVectorImpl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
