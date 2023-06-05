#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod clang_importer_options;
pub mod lang_options;
pub mod source_loc;
pub mod source_manager;
pub mod type_checker_options;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["swift", "basic"];
    let path_descendants = &[
        "clang_importer_options",
        "lang_options",
        "source_loc",
        "source_manager",
        "type_checker_options",
    ];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
