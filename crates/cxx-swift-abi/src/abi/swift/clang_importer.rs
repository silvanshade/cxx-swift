#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod clang_importer;
pub mod effective_clang_context;
pub mod serialized_swift_name;
pub mod swift_lookup_table;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["swift", "clang_importer"];
    let path_descendants = &[
        "clang_importer",
        "effective_clang_context",
        "serialized_swift_name",
        "swift_lookup_table",
    ];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
