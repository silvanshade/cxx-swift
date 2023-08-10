use cxx_llvm_build_common::prelude::*;
use std::path::{Path, PathBuf};

fn process_cxx(out_dir: &Path) -> BoxResult<()> {
    let cargo_pkg_name = "cxx-swift";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let clang_dirs = cxx_clang_build::Dirs::new(cargo_pkg_name, &llvm_dirs)?;
    let swift_dirs = cxx_swift_build::Dirs::new(cargo_pkg_name, &llvm_dirs, &clang_dirs)?;
    let rust_source_files = &[
        out_dir.join("src/auto/swift/ast/ast_context.rs"),
        out_dir.join("src/auto/swift/ast/ast_walker_base.rs"),
        out_dir.join("src/auto/swift/ast/ast_walker_rust.rs"),
        out_dir.join("src/auto/swift/ast/ast_walker.rs"),
        out_dir.join("src/auto/swift/ast/ast_walker/parent_ty.rs"),
        out_dir.join("src/auto/swift/ast/ast_walker/pre_walk_result_expr.rs"),
        out_dir.join("src/auto/swift/ast/decl.rs"),
        out_dir.join("src/auto/swift/ast/dependency_tracker.rs"),
        out_dir.join("src/auto/swift/ast/diagnostic_engine.rs"),
        out_dir.join("src/auto/swift/ast/expr.rs"),
        out_dir.join("src/auto/swift/ast/identifier.rs"),
        out_dir.join("src/auto/swift/ast/identifier/small_vector.rs"),
        out_dir.join("src/auto/swift/ast/identifier/small_vector_impl.rs"),
        out_dir.join("src/auto/swift/ast/import_path/module.rs"),
        out_dir.join("src/auto/swift/ast/import_path/module/builder.rs"),
        out_dir.join("src/auto/swift/ast/module_decl.rs"),
        out_dir.join("src/auto/swift/ast/pattern.rs"),
        out_dir.join("src/auto/swift/ast/search_path_options.rs"),
        out_dir.join("src/auto/swift/ast/sil_options.rs"),
        out_dir.join("src/auto/swift/ast/stmt.rs"),
        out_dir.join("src/auto/swift/ast/type_repr.rs"),
        out_dir.join("src/auto/swift/basic/clang_importer_options.rs"),
        out_dir.join("src/auto/swift/basic/lang_options.rs"),
        out_dir.join("src/auto/swift/basic/source_loc.rs"),
        out_dir.join("src/auto/swift/basic/source_manager.rs"),
        out_dir.join("src/auto/swift/basic/type_checker_options.rs"),
        out_dir.join("src/auto/swift/clang_importer/clang_importer.rs"),
        out_dir.join("src/auto/swift/clang_importer/effective_clang_context.rs"),
        out_dir.join("src/auto/swift/clang_importer/serialized_swift_name.rs"),
        out_dir.join("src/auto/swift/clang_importer/serialized_swift_name/small_vector.rs"),
        out_dir.join("src/auto/swift/clang_importer/serialized_swift_name/small_vector_boxed.rs"),
        out_dir.join("src/auto/swift/clang_importer/serialized_swift_name/small_vector_impl.rs"),
        out_dir.join("src/auto/swift/clang_importer/swift_lookup_table.rs"),
        out_dir.join("src/auto/swift/clang_importer/swift_lookup_table/single_entry.rs"),
        out_dir.join("src/auto/swift/clang_importer/swift_lookup_table/single_entry/small_vector.rs"),
        out_dir.join("src/auto/swift/clang_importer/swift_lookup_table/single_entry/small_vector_boxed.rs"),
        out_dir.join("src/auto/swift/clang_importer/swift_lookup_table/single_entry/small_vector_impl.rs"),
        out_dir.join("src/auto/swift/symbol_graph_gen/symbol_graph_options.rs"),
        PathBuf::from("src/gen/swift/ast/ast_context.rs"),
        PathBuf::from("src/gen/swift/ast/ast_walker_base.rs"),
        PathBuf::from("src/gen/swift/ast/ast_walker_rust.rs"),
        PathBuf::from("src/gen/swift/ast/ast_walker.rs"),
        PathBuf::from("src/gen/swift/ast/ast_walker/parent_ty.rs"),
        PathBuf::from("src/gen/swift/ast/ast_walker/pre_walk_result_expr.rs"),
        PathBuf::from("src/gen/swift/ast/decl.rs"),
        PathBuf::from("src/gen/swift/ast/diagnostic_engine.rs"),
        PathBuf::from("src/gen/swift/ast/expr.rs"),
        PathBuf::from("src/gen/swift/ast/identifier.rs"),
        PathBuf::from("src/gen/swift/ast/identifier/small_vector.rs"),
        PathBuf::from("src/gen/swift/ast/identifier/small_vector_impl.rs"),
        PathBuf::from("src/gen/swift/ast/import_path/module.rs"),
        PathBuf::from("src/gen/swift/ast/import_path/module/builder.rs"),
        PathBuf::from("src/gen/swift/ast/module_decl.rs"),
        PathBuf::from("src/gen/swift/ast/pattern.rs"),
        PathBuf::from("src/gen/swift/ast/search_path_options.rs"),
        PathBuf::from("src/gen/swift/ast/sil_options.rs"),
        PathBuf::from("src/gen/swift/ast/stmt.rs"),
        PathBuf::from("src/gen/swift/ast/type_repr.rs"),
        PathBuf::from("src/gen/swift/basic/clang_importer_options.rs"),
        PathBuf::from("src/gen/swift/basic/lang_options.rs"),
        PathBuf::from("src/gen/swift/basic/llvm_initialize.rs"),
        PathBuf::from("src/gen/swift/basic/source_loc.rs"),
        PathBuf::from("src/gen/swift/basic/source_manager.rs"),
        PathBuf::from("src/gen/swift/basic/type_checker_options.rs"),
        PathBuf::from("src/gen/swift/clang_importer/clang_importer.rs"),
        PathBuf::from("src/gen/swift/clang_importer/effective_clang_context.rs"),
        PathBuf::from("src/gen/swift/clang_importer/serialized_swift_name.rs"),
        PathBuf::from("src/gen/swift/clang_importer/serialized_swift_name/small_vector.rs"),
        PathBuf::from("src/gen/swift/clang_importer/serialized_swift_name/small_vector_boxed.rs"),
        PathBuf::from("src/gen/swift/clang_importer/serialized_swift_name/small_vector_impl.rs"),
        PathBuf::from("src/gen/swift/clang_importer/swift_lookup_table.rs"),
        PathBuf::from("src/gen/swift/clang_importer/swift_lookup_table/single_entry.rs"),
        PathBuf::from("src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector.rs"),
        PathBuf::from("src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector_boxed.rs"),
        PathBuf::from("src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector_impl.rs"),
        PathBuf::from("src/gen/swift/symbol_graph_gen/symbol_graph_options.rs"),
    ];
    let files: &[&str] = &[
        "cxx/lib/swift/AST/ASTWalkerBase.cxx",
        "cxx/lib/swift/AST/ASTWalkerRust.cxx",
    ];
    let output = "cxx-swift";
    cxx_swift_build::cxx_build(&llvm_dirs, &clang_dirs, &swift_dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=src/gen");
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    cxx_swift_auto::auto::process_artifacts(&out_dir)?;
    process_cxx(&out_dir)?;
    Ok(())
}
