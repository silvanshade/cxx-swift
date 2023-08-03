use cxx_llvm_build_common::prelude::*;
use std::path::PathBuf;

pub fn project_dir() -> BoxResult<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let project_dir = std::path::PathBuf::from(cargo_manifest_dir);
    Ok(project_dir)
}

fn process_cxx() -> BoxResult<()> {
    let cargo_pkg_name = "cxx-swift-abi";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let clang_dirs = cxx_clang_build::Dirs::new(cargo_pkg_name, &llvm_dirs)?;
    let swift_dirs = cxx_swift_build::Dirs::new(cargo_pkg_name, &llvm_dirs, &clang_dirs)?;
    let includes = &[
        llvm_dirs.llvm_project.join("llvm/include"),
        llvm_dirs.llvm_cmake_build.join("include"),
        clang_dirs.clang_project.join("include"),
        clang_dirs.clang_cmake_build.join("include"),
        swift_dirs.swift_project.clone(),
        swift_dirs.swift_project.join("swift/include"),
        swift_dirs.swift_project.join("swift/stdlib/public/SwiftShims"),
        swift_dirs.swift_cmake_build.join("include"),
    ];
    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(PathBuf::as_path));
    let rust_source_files = &[
        "src/abi/swift/ast/ast_context.rs",
        "src/abi/swift/ast/ast_walker_base.rs",
        "src/abi/swift/ast/ast_walker_rust.rs",
        "src/abi/swift/ast/ast_walker.rs",
        "src/abi/swift/ast/ast_walker/parent_ty.rs",
        "src/abi/swift/ast/ast_walker/pre_walk_result_expr.rs",
        "src/abi/swift/ast/decl.rs",
        "src/abi/swift/ast/dependency_tracker.rs",
        "src/abi/swift/ast/diagnostic_engine.rs",
        "src/abi/swift/ast/expr.rs",
        "src/abi/swift/ast/identifier.rs",
        "src/abi/swift/ast/identifier/small_vector.rs",
        "src/abi/swift/ast/identifier/small_vector_impl.rs",
        "src/abi/swift/ast/import_path/module.rs",
        "src/abi/swift/ast/import_path/module/builder.rs",
        "src/abi/swift/ast/module_decl.rs",
        "src/abi/swift/ast/pattern.rs",
        "src/abi/swift/ast/search_path_options.rs",
        "src/abi/swift/ast/sil_options.rs",
        "src/abi/swift/ast/stmt.rs",
        "src/abi/swift/ast/type_repr.rs",
        "src/abi/swift/basic/clang_importer_options.rs",
        "src/abi/swift/basic/lang_options.rs",
        "src/abi/swift/basic/source_loc.rs",
        "src/abi/swift/basic/source_manager.rs",
        "src/abi/swift/basic/type_checker_options.rs",
        "src/abi/swift/clang_importer/clang_importer.rs",
        "src/abi/swift/clang_importer/effective_clang_context.rs",
        "src/abi/swift/clang_importer/serialized_swift_name.rs",
        "src/abi/swift/clang_importer/serialized_swift_name/small_vector.rs",
        "src/abi/swift/clang_importer/serialized_swift_name/small_vector_boxed.rs",
        "src/abi/swift/clang_importer/serialized_swift_name/small_vector_impl.rs",
        "src/abi/swift/clang_importer/swift_lookup_table.rs",
        "src/abi/swift/clang_importer/swift_lookup_table/single_entry.rs",
        "src/abi/swift/clang_importer/swift_lookup_table/single_entry/small_vector.rs",
        "src/abi/swift/clang_importer/swift_lookup_table/single_entry/small_vector_boxed.rs",
        "src/abi/swift/clang_importer/swift_lookup_table/single_entry/small_vector_impl.rs",
        "src/abi/swift/symbol_graph_gen/symbol_graph_options.rs",
        "src/proxy/ast_walker_dyn.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-swift-abi";
    cxx_swift_build::cxx_build(&llvm_dirs, &clang_dirs, &swift_dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=abi");
    println!("cargo:rerun-if-changed=cxx");
    let project_dir = project_dir()?;
    let abi_dir = project_dir.join("abi");
    let abi_crate_src_dir = project_dir.join("src");
    cxx_memory_abi::process_artifacts(&abi_dir, &abi_crate_src_dir)?;
    process_cxx()?;
    Ok(())
}
