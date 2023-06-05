use crate::{
    ffi::swift::{
        ast::{diagnostic_engine::DiagnosticEngine, search_path_options::SearchPathOptions, sil_options::SilOptions},
        basic::{
            clang_importer_options::ClangImporterOptions,
            lang_options::LangOptions,
            type_checker_options::TypeCheckerOptions,
        },
        symbol_graph_gen::symbol_graph_options::SymbolGraphOptions,
    },
    gen::swift::ast::ast_context,
};
use core::pin::Pin;
use cxx_llvm::llvm::StringRef;

pub use crate::abi::swift::ast::ast_context::AstContext;

impl<'cfg> AstContext<'cfg> {
    #[inline]
    pub fn get<'source_manager: 'cfg>(
        lang_options: Pin<&'cfg mut LangOptions>,
        type_checker_options: Pin<&'cfg mut TypeCheckerOptions>,
        sil_options: Pin<&'cfg mut SilOptions>,
        search_path_options: Pin<&'cfg mut SearchPathOptions>,
        clang_importer_options: Pin<&'cfg mut ClangImporterOptions>,
        symbol_graph_options: Pin<&'cfg mut SymbolGraphOptions>,
        diagnostic_engine: Pin<&'cfg mut DiagnosticEngine<'source_manager>>,
        pre_module_import_callback: for<'s> fn(StringRef<'s>, bool) -> bool,
    ) -> cxx::UniquePtr<Self> {
        ast_context::get(
            lang_options,
            type_checker_options,
            sil_options,
            search_path_options,
            clang_importer_options,
            symbol_graph_options,
            diagnostic_engine,
            pre_module_import_callback,
        )
    }
}
