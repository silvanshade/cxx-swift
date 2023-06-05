#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod module;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["swift", "ast", "import_path"];
    let path_descendants = &["module"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
