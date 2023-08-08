#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry.hxx");
        include!("cxx-swift-auto/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorImpl.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry"]
        type SwiftLookupTableSingleEntry<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::SmallVectorImpl<
                'ctx,
            >;
    }

    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl"]
    unsafe extern "C++" {
        fn as_slice<'this, 'ctx>(This: &'this SmallVectorImpl<'ctx>) -> &'this [SwiftLookupTableSingleEntry<'ctx>];

        unsafe fn as_mut_slice<'this, 'ctx>(
            This: Pin<&'this mut SmallVectorImpl<'ctx>>,
        ) -> &'this mut [SwiftLookupTableSingleEntry<'ctx>];
    }
}
pub(crate) use self::ffi::*;
