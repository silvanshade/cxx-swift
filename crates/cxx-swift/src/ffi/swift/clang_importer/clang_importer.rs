use crate::{
    ffi::swift::{
        ast::{
            ast_context::AstContext,
            dependency_tracker::DependencyTracker,
            identifier::Identifier,
            import_path,
            module_decl::ModuleDecl,
        },
        basic::source_loc::SourceLoc,
        clang_importer::swift_lookup_table::SwiftLookupTable,
    },
    gen::swift::clang_importer::clang_importer,
};
use core::pin::Pin;
use cxx_clang::clang::basic::module::Module as ClangModule;
use cxx_llvm::llvm;

pub use crate::auto::swift::clang_importer::clang_importer::ClangImporter;

impl<'ctx> ClangImporter<'ctx> {
    #[inline]
    pub fn create<'cfg: 'ctx>(
        ast_context: Pin<&'ctx mut AstContext<'cfg>>,
        swift_pch_hash: Option<Pin<&'ctx mut cxx::String>>,
        dependency_tracker: Option<Pin<&'ctx mut DependencyTracker>>,
    ) -> cxx::UniquePtr<Self> {
        let swift_pch_hash = swift_pch_hash
            .map(|ptr| unsafe { Pin::into_inner_unchecked(ptr) } as *mut cxx::String)
            .unwrap_or_else(|| core::ptr::null_mut());
        let dependency_tracker = dependency_tracker
            .map(|ptr| unsafe { Pin::into_inner_unchecked(ptr) } as *mut DependencyTracker)
            .unwrap_or_else(|| core::ptr::null_mut());
        unsafe { clang_importer::create(ast_context, swift_pch_hash, dependency_tracker) }
    }

    #[inline]
    pub fn collect_visible_top_level_module_names(
        self: Pin<&mut Self>,
        module_names: Pin<&mut llvm::SmallVectorImpl<Identifier>>,
    ) {
        let module_names = unsafe { core::mem::transmute(module_names) };
        clang_importer::collect_visible_top_level_module_names(self, module_names)
    }

    #[inline]
    pub fn load_module(
        self: Pin<&mut Self>,
        source_loc: SourceLoc,
        module_path: import_path::module::Module<'ctx>,
        allow_memory_cache: Option<bool>,
    ) -> Option<&'ctx ModuleDecl<'ctx>> {
        let allow_memory_cache = allow_memory_cache.unwrap_or(true);
        let ptr = clang_importer::load_module(self, source_loc, module_path, allow_memory_cache);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn emit_bridging_pch(self: Pin<&mut Self>, header: &std::path::Path, output_pch: &std::path::Path) -> bool {
        let header_path = llvm::StringRef::from(header);
        let output_pch_path = llvm::StringRef::from(output_pch);
        clang_importer::emit_bridging_pch(self, header_path, output_pch_path)
    }

    #[inline]
    pub fn can_read_pch(&self, pch: &std::path::Path) -> bool {
        let pch_path = llvm::StringRef::from(pch);
        clang_importer::can_read_pch(self, pch_path)
    }

    #[inline]
    pub fn find_lookup_table_for_module(
        &self,
        clang_module: &ClangModule<'ctx>,
    ) -> Option<Pin<&mut SwiftLookupTable<'ctx>>> {
        let ptr = clang_importer::find_lookup_table_for_module(self, clang_module);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Pin::new_unchecked(&mut *ptr) })
        }
    }
}
