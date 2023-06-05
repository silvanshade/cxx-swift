#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod identifier;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["llvm", "adt", "small_vector", "swift", "ast"];
    let path_descendants = &["identifier"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
