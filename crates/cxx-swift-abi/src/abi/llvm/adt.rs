#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod small_vector;
pub mod small_vector_impl;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["llvm", "adt"];
    let path_descendants = &["small_vector", "small_vector_impl"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
