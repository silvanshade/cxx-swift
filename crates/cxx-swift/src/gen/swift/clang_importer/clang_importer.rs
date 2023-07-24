#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Basic/Module.hxx");
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/StringRef.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTContext.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/DependencyTracker.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/Identifier.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/Identifier/SmallVectorImpl.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ImportPath/Module.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ModuleDecl.hxx");
        include!("cxx-swift-abi/cxx/include/swift/Basic/SourceLoc.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/ClangImporter.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_context"]
        #[cxx_name = "ASTContext"]
        type AstContext<'cfg> = crate::ffi::swift::ast::ast_context::AstContext<'cfg>;

        #[namespace = "cxx_swift::swift::clang_importer::clang_importer"]
        type ClangImporter<'ctx> = crate::ffi::swift::clang_importer::clang_importer::ClangImporter<'ctx>;

        #[namespace = "cxx_clang::clang::basic::module"]
        #[cxx_name = "Module"]
        type ClangModule<'ctx> = cxx_clang::clang::basic::module::Module<'ctx>;

        #[namespace = "cxx_swift::swift::ast::dependency_tracker"]
        type DependencyTracker = crate::ffi::swift::ast::dependency_tracker::DependencyTracker;

        #[namespace = "cxx_swift::swift::ast::identifier::small_vector_impl"]
        #[cxx_name = "SmallVectorImpl"]
        type IdentifierSmallVectorImpl<'ctx> =
            crate::ffi::swift::ast::identifier::small_vector_impl::SmallVectorImpl<'ctx>;

        #[namespace = "cxx_swift::swift::ast::import_path::module"]
        #[cxx_name = "Module"]
        type ImportPathModule<'ctx> = crate::ffi::swift::ast::import_path::module::Module<'ctx>;

        #[namespace = "cxx_swift::swift::ast::module_decl"]
        type ModuleDecl<'ctx> = crate::ffi::swift::ast::module_decl::ModuleDecl<'ctx>;

        #[namespace = "cxx_llvm::llvm::adt::string_ref"]
        type StringRef<'a> = cxx_llvm::llvm::StringRef<'a>;

        #[namespace = "cxx_swift::swift::basic::source_loc"]
        type SourceLoc = crate::ffi::swift::basic::source_loc::SourceLoc;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table"]
        type SwiftLookupTable<'ctx> = crate::ffi::swift::clang_importer::swift_lookup_table::SwiftLookupTable<'ctx>;
    }

    impl<'cfg> UniquePtr<ClangImporter<'cfg>> {
    }

    #[namespace = "cxx_swift::swift::clang_importer::clang_importer"]
    unsafe extern "C++" {
        unsafe fn create<'ctx, 'cfg>(
            ast_context: Pin<&'ctx mut AstContext<'cfg>>,
            swift_pch_hash: *mut CxxString,
            dependency_tracker: *mut DependencyTracker,
        ) -> UniquePtr<ClangImporter<'ctx>>;

        fn collect_visible_top_level_module_names<'ctx>(
            This: Pin<&mut ClangImporter<'ctx>>,
            module_names: Pin<&mut IdentifierSmallVectorImpl<'ctx>>,
        );

        fn load_module<'ctx>(
            This: Pin<&mut ClangImporter<'ctx>>,
            source_loc: SourceLoc,
            module_path: ImportPathModule<'ctx>,
            allow_memory_cache: bool,
        ) -> *const ModuleDecl<'ctx>;

        fn emit_bridging_pch(This: Pin<&mut ClangImporter>, header_path: StringRef, output_pch_path: StringRef)
        -> bool;

        fn can_read_pch(This: &ClangImporter, pch_path: StringRef) -> bool;

        fn find_lookup_table_for_module<'ctx>(
            This: &ClangImporter<'ctx>,
            clang_module: &ClangModule<'ctx>,
        ) -> *mut SwiftLookupTable<'ctx>;
    }
}
pub(crate) use self::ffi::*;
