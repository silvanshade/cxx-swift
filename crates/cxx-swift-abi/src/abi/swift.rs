#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod ast;
pub mod basic;
pub mod clang_importer;
pub mod symbol_graph_gen;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["swift"];
    let path_descendants = &["ast", "basic", "clang_importer", "symbol_graph_gen"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
