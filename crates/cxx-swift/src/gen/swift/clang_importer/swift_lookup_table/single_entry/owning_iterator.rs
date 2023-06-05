#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/OwningIterator.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry"]
        type SwiftLookupTableSingleEntry<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator"]
        type SwiftLookupTableSingleEntryOwningIterator<'ctx> = crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator::SwiftLookupTableSingleEntryOwningIterator<'ctx>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator"]
    unsafe extern "C++" {
        fn next<'ctx>(
            This: Pin<&mut SwiftLookupTableSingleEntryOwningIterator<'ctx>>,
        ) -> *mut SwiftLookupTableSingleEntry<'ctx>;
    }
}
pub(crate) use self::ffi::*;
