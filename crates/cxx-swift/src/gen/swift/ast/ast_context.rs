#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/StringRef.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTContext.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/DiagnosticEngine.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/SearchPathOptions.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/SILOptions.hxx");
        include!("cxx-swift-abi/cxx/include/swift/Basic/ClangImporterOptions.hxx");
        include!("cxx-swift-abi/cxx/include/swift/Basic/LangOptions.hxx");
        include!("cxx-swift-abi/cxx/include/swift/Basic/TypeCheckerOptions.hxx");
        include!("cxx-swift-abi/cxx/include/swift/SymbolGraphGen/SymbolGraphOptions.hxx");

        #[namespace = "cxx_swift::swift::ast::ast_context"]
        #[cxx_name = "ASTContext"]
        type AstContext<'cfg> = crate::ffi::swift::ast::ast_context::AstContext<'cfg>;

        #[namespace = "cxx_swift::swift::basic::clang_importer_options"]
        type ClangImporterOptions = crate::ffi::swift::basic::clang_importer_options::ClangImporterOptions;

        #[namespace = "cxx_swift::swift::ast::diagnostic_engine"]
        type DiagnosticEngine<'source_manager> =
            crate::ffi::swift::ast::diagnostic_engine::DiagnosticEngine<'source_manager>;

        #[namespace = "cxx_swift::swift::basic::lang_options"]
        type LangOptions = crate::ffi::swift::basic::lang_options::LangOptions;

        #[namespace = "cxx_swift::swift::ast::search_path_options"]
        type SearchPathOptions = crate::ffi::swift::ast::search_path_options::SearchPathOptions;

        #[namespace = "cxx_swift::swift::ast::sil_options"]
        #[cxx_name = "SILOptions"]
        type SilOptions = crate::ffi::swift::ast::sil_options::SilOptions;

        #[namespace = "cxx_llvm::llvm::adt::string_ref"]
        type StringRef<'a> = cxx_llvm::llvm::adt::string_ref::StringRef<'a>;

        #[namespace = "cxx_swift::swift::symbol_graph_gen::symbol_graph_options"]
        type SymbolGraphOptions = crate::ffi::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;

        #[namespace = "cxx_swift::swift::basic::type_checker_options"]
        type TypeCheckerOptions = crate::ffi::swift::basic::type_checker_options::TypeCheckerOptions;
    }

    impl<'cfg> UniquePtr<AstContext<'cfg>> {
    }

    #[namespace = "cxx_swift::swift::ast::ast_context"]
    unsafe extern "C++" {
        fn get<'cfg, 'source_manager, 's>(
            lang_options: Pin<&'cfg mut LangOptions>,
            type_checker_options: Pin<&'cfg mut TypeCheckerOptions>,
            sil_options: Pin<&'cfg mut SilOptions>,
            search_path_options: Pin<&'cfg mut SearchPathOptions>,
            clang_importer_options: Pin<&'cfg mut ClangImporterOptions>,
            symbol_graph_options: Pin<&'cfg mut SymbolGraphOptions>,
            diagnostic_engine: Pin<&'cfg mut DiagnosticEngine<'source_manager>>,
            pre_module_import_callback: fn(StringRef<'s>, bool) -> bool,
        ) -> UniquePtr<AstContext<'cfg>>;
    }
}
pub(crate) use self::ffi::*;
