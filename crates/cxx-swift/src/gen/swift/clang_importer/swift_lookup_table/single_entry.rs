pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/AST/Decl/NamedDecl.hxx");
        include!("cxx-clang-abi/cxx/include/clang/Lex/MacroInfo.hxx");
        include!("cxx-clang-abi/cxx/include/clang/Lex/MacroInfo/ModuleMacro.hxx");
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry.hxx");

        #[namespace = "cxx_clang::clang::lex::macro_info"]
        #[cxx_name = "MacroInfo"]
        type ClangMacroInfo<'ctx> = cxx_clang::clang::lex::macro_info::MacroInfo<'ctx>;

        #[namespace = "cxx_clang::clang::lex::macro_info::module_macro"]
        #[cxx_name = "ModuleMacro"]
        type ClangModuleMacro<'ctx> = cxx_clang::clang::lex::macro_info::module_macro::ModuleMacro<'ctx>;

        #[namespace = "cxx_clang::clang::ast::decl::named_decl"]
        #[cxx_name = "NamedDecl"]
        type ClangNamedDecl<'ctx> = cxx_clang::clang::ast::decl::named_decl::NamedDecl<'ctx>;

        #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry"]
        type SwiftLookupTableSingleEntry<'ctx> =
            crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry<'ctx>;
    }

    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry"]
    unsafe extern "C++" {
        fn cast_as_named_decl<'ctx>(This: &SwiftLookupTableSingleEntry<'ctx>) -> *const ClangNamedDecl<'ctx>;

        fn cast_as_macro_info<'ctx>(This: &SwiftLookupTableSingleEntry<'ctx>) -> *const ClangMacroInfo<'ctx>;

        fn cast_as_module_macro<'ctx>(This: &SwiftLookupTableSingleEntry<'ctx>) -> *const ClangModuleMacro<'ctx>;
    }
}
pub(crate) use self::ffi::*;
