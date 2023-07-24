pub(crate) mod single_entry;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/EffectiveClangContext.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVectorBoxed.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorBoxed.hxx");

        #[namespace = "cxx_swift::swift::clang_importer::effective_clang_context"]
        type EffectiveClangContext = crate::ffi::swift::clang_importer::effective_clang_context::EffectiveClangContext;

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
        type SerializedSwiftName<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_boxed"]
        #[cxx_name = "SmallVectorBoxed"]
        type SerializedSwiftNameSmallVectorBoxed<'ctx> =
            crate::ffi::swift::clang_importer::serialized_swift_name::small_vector_boxed::SmallVectorBoxed<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table"]
        type SwiftLookupTable<'ctx> = crate::ffi::swift::clang_importer::swift_lookup_table::SwiftLookupTable<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed"]
        #[cxx_name = "SmallVectorBoxed"]
        type SwiftLookupTableSingleEntrySmallVectorBoxed<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed::SmallVectorBoxed<
                'ctx,
            >;
    }

    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table"]
    unsafe extern "C++" {
        unsafe fn lookup<'ctx>(
            This: &SwiftLookupTable<'ctx>,
            base_name: SerializedSwiftName<'ctx>,
            search_context: EffectiveClangContext,
            out: *mut SwiftLookupTableSingleEntrySmallVectorBoxed<'ctx>,
        );

        unsafe fn all_base_names<'ctx>(
            This: &SwiftLookupTable<'ctx>,
            out: *mut SerializedSwiftNameSmallVectorBoxed<'ctx>,
        );

        fn deserialize_all<'ctx>(This: Pin<&mut SwiftLookupTable<'ctx>>);

        fn dump<'ctx>(This: Pin<&mut SwiftLookupTable<'ctx>>);
    }
}
pub(crate) use self::ffi::*;
