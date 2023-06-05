pub(crate) mod owning_iterator;

use crate::gen::swift::clang_importer::swift_lookup_table::single_entry;
use cxx_clang::clang::{
    ast::decl::named_decl::NamedDecl as ClangNamedDecl,
    lex::macro_info::{module_macro::ModuleMacro as ClangModuleMacro, MacroInfo as ClangMacroInfo},
};

pub use crate::abi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry;

impl<'ctx> SwiftLookupTableSingleEntry<'ctx> {
    #[inline]
    pub fn cast_as_named_decl(&self) -> Option<&ClangNamedDecl<'ctx>> {
        let ptr = single_entry::cast_as_named_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_macro_info(&self) -> Option<&ClangMacroInfo<'ctx>> {
        let ptr = single_entry::cast_as_macro_info(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_module_macro(&self) -> Option<&ClangModuleMacro<'ctx>> {
        let ptr = single_entry::cast_as_module_macro(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}
