#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorBoxed.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorImpl.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed"]
        type SmallVectorBoxed<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed::SmallVectorBoxed<
                'ctx,
            >;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::SmallVectorImpl<
                'ctx,
            >;
    }

    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed"]
    unsafe extern "C++" {
        fn as_ref_small_vector_impl<'this, 'ctx>(This: &'this SmallVectorBoxed<'ctx>) -> &'this SmallVectorImpl<'ctx>;

        fn as_pin_small_vector_impl<'this, 'ctx>(
            This: Pin<&'this mut SmallVectorBoxed<'ctx>>,
        ) -> Pin<&'this mut SmallVectorImpl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
