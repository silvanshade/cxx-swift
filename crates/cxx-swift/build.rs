use cxx_llvm_build_common::prelude::*;

fn process_cxx() -> BoxResult<()> {
    let cargo_pkg_name = "cxx-clang";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let clang_dirs = cxx_clang_build::Dirs::new(cargo_pkg_name, &llvm_dirs)?;
    let swift_dirs = cxx_swift_build::Dirs::new(cargo_pkg_name, &llvm_dirs, &clang_dirs)?;
    let rust_source_files: &[&str] = &[
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
        "src/gen/swift/ast/ast_context.rs",
        "src/gen/swift/ast/ast_walker_base.rs",
        "src/gen/swift/ast/ast_walker_rust.rs",
        "src/gen/swift/ast/ast_walker.rs",
        "src/gen/swift/ast/ast_walker/parent_ty.rs",
        "src/gen/swift/ast/ast_walker/pre_walk_result_expr.rs",
        "src/gen/swift/ast/decl.rs",
        "src/gen/swift/ast/diagnostic_engine.rs",
        "src/gen/swift/ast/expr.rs",
        "src/gen/swift/ast/identifier.rs",
        "src/gen/swift/ast/identifier/small_vector.rs",
        "src/gen/swift/ast/identifier/small_vector_impl.rs",
        "src/gen/swift/ast/import_path/module.rs",
        "src/gen/swift/ast/import_path/module/builder.rs",
        "src/gen/swift/ast/module_decl.rs",
        "src/gen/swift/ast/pattern.rs",
        "src/gen/swift/ast/search_path_options.rs",
        "src/gen/swift/ast/sil_options.rs",
        "src/gen/swift/ast/stmt.rs",
        "src/gen/swift/ast/type_repr.rs",
        "src/gen/swift/basic/clang_importer_options.rs",
        "src/gen/swift/basic/lang_options.rs",
        "src/gen/swift/basic/llvm_initialize.rs",
        "src/gen/swift/basic/source_loc.rs",
        "src/gen/swift/basic/source_manager.rs",
        "src/gen/swift/basic/type_checker_options.rs",
        "src/gen/swift/clang_importer/clang_importer.rs",
        "src/gen/swift/clang_importer/effective_clang_context.rs",
        "src/gen/swift/clang_importer/serialized_swift_name.rs",
        "src/gen/swift/clang_importer/serialized_swift_name/small_vector.rs",
        "src/gen/swift/clang_importer/serialized_swift_name/small_vector_boxed.rs",
        "src/gen/swift/clang_importer/serialized_swift_name/small_vector_impl.rs",
        "src/gen/swift/clang_importer/swift_lookup_table.rs",
        "src/gen/swift/clang_importer/swift_lookup_table/single_entry.rs",
        "src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector.rs",
        "src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector_boxed.rs",
        "src/gen/swift/clang_importer/swift_lookup_table/single_entry/small_vector_impl.rs",
        "src/gen/swift/symbol_graph_gen/symbol_graph_options.rs",
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
    println!("cargo:rerun-if-changed=../cxx-swift-abi");
    cxx_swift_abi::abi::process_artifacts()?;
    process_cxx()?;
    Ok(())
}
