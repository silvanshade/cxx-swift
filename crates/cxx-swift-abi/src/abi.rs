#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod llvm;
pub mod swift;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &[];
    let path_descendants = &["llvm", "swift"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
pub fn process_artifacts() -> ::cxx_memory_abi::BoxResult<()> {
    self::write_module()?;
    self::llvm::write_module()?;
    self::llvm::adt::write_module()?;
    self::llvm::adt::small_vector::write_module()?;
    self::llvm::adt::small_vector::swift::write_module()?;
    self::llvm::adt::small_vector::swift::ast::write_module()?;
    self::llvm::adt::small_vector::swift::ast::identifier::write_module()?;
    self::llvm::adt::small_vector_impl::write_module()?;
    self::llvm::adt::small_vector_impl::swift::write_module()?;
    self::llvm::adt::small_vector_impl::swift::ast::write_module()?;
    self::llvm::adt::small_vector_impl::swift::ast::identifier::write_module()?;
    self::swift::write_module()?;
    self::swift::ast::write_module()?;
    self::swift::ast::ast_context::write_module()?;
    self::swift::ast::ast_walker::write_module()?;
    self::swift::ast::ast_walker::parent_ty::write_module()?;
    self::swift::ast::ast_walker::pre_walk_result_expr::write_module()?;
    self::swift::ast::ast_walker_base::write_module()?;
    self::swift::ast::ast_walker_rust::write_module()?;
    self::swift::ast::decl::write_module()?;
    self::swift::ast::dependency_tracker::write_module()?;
    self::swift::ast::diagnostic_engine::write_module()?;
    self::swift::ast::expr::write_module()?;
    self::swift::ast::identifier::write_module()?;
    self::swift::ast::import_path::write_module()?;
    self::swift::ast::import_path::module::write_module()?;
    self::swift::ast::import_path::module::builder::write_module()?;
    self::swift::ast::module_decl::write_module()?;
    self::swift::ast::pattern::write_module()?;
    self::swift::ast::search_path_options::write_module()?;
    self::swift::ast::sil_options::write_module()?;
    self::swift::ast::stmt::write_module()?;
    self::swift::ast::type_repr::write_module()?;
    self::swift::basic::write_module()?;
    self::swift::basic::clang_importer_options::write_module()?;
    self::swift::basic::lang_options::write_module()?;
    self::swift::basic::source_loc::write_module()?;
    self::swift::basic::source_manager::write_module()?;
    self::swift::basic::type_checker_options::write_module()?;
    self::swift::clang_importer::write_module()?;
    self::swift::clang_importer::clang_importer::write_module()?;
    self::swift::clang_importer::effective_clang_context::write_module()?;
    self::swift::clang_importer::serialized_swift_name::write_module()?;
    self::swift::clang_importer::serialized_swift_name::owning_iterator::write_module()?;
    self::swift::clang_importer::swift_lookup_table::write_module()?;
    self::swift::clang_importer::swift_lookup_table::single_entry::write_module()?;
    self::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator::write_module()?;
    self::swift::symbol_graph_gen::write_module()?;
    self::swift::symbol_graph_gen::symbol_graph_options::write_module()?;
    Ok(())
}
