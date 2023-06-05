#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod ast_context;
pub mod ast_walker;
pub mod ast_walker_base;
pub mod ast_walker_rust;
pub mod decl;
pub mod dependency_tracker;
pub mod diagnostic_engine;
pub mod expr;
pub mod identifier;
pub mod import_path;
pub mod module_decl;
pub mod pattern;
pub mod search_path_options;
pub mod sil_options;
pub mod stmt;
pub mod type_repr;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["swift", "ast"];
    let path_descendants = &[
        "ast_context",
        "ast_walker",
        "ast_walker_base",
        "ast_walker_rust",
        "decl",
        "dependency_tracker",
        "diagnostic_engine",
        "expr",
        "identifier",
        "import_path",
        "module_decl",
        "pattern",
        "search_path_options",
        "sil_options",
        "stmt",
        "type_repr",
    ];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
