use crate::common::*;
use cxx_llvm::llvm;
use cxx_swift::swift;
use indoc::indoc;

#[test]
fn test() -> BoxResult<()> {
    let temp = tempfile::tempdir()?;

    // Initialize default clang importer options.
    let_cxx!(mut clang_importer_options = swift::ClangImporterOptions::new());

    // Create a cache subdirectory for the modules and PCH.
    let cache = temp.path().join("cache");
    std::fs::create_dir(&cache)?;
    clang_importer_options.as_mut().set_module_cache_path(&cache);
    clang_importer_options
        .as_mut()
        .set_precompiled_header_output_dir(&cache);

    // Create the includes.
    let include = temp.path().join("include");
    std::fs::create_dir(&include)?;

    let module_dot_modulemap = include.join("module.modulemap");
    let a_dot_h = include.join("A.h");
    let bridging_dot_h = temp.path().join("bridging.h");

    clang_importer_options
        .as_mut()
        .modify_extra_args_push_back("-nosysteminc");
    {
        let include = include
            .as_os_str()
            .to_str()
            .expect("path should be a valid UTF-8 string");
        clang_importer_options
            .as_mut()
            .modify_extra_args_push_back(&format!("-I{include}"));
    }
    {
        std::fs::write(&module_dot_modulemap, indoc! {r#"
            module A {
                header "A.h"
            }
        "#})?;
        std::fs::write(&a_dot_h, indoc! {r#"
            int foo(void);
        "#})?;
        std::fs::write(&bridging_dot_h, indoc! {r#"
            #import <A.h>
        "#})?;
    }

    // Create a bridging header.
    clang_importer_options.as_mut().set_bridging_header(&bridging_dot_h);

    // Set up the importer.
    let_cxx!(mut lang_options = swift::LangOptions::new());
    {
        let_cxx!(target = llvm::Triple::from(("x86_64", "apple", "darwin")));
        lang_options.as_mut().set_target(target)?;
    }

    let_cxx!(mut sil_options = swift::SilOptions::new());
    let_cxx!(mut type_checker_options = swift::TypeCheckerOptions::new());

    swift::initialize_llvm();

    let_cxx!(mut search_path_options = swift::SearchPathOptions::new());
    let_cxx!(mut symbol_graph_options = swift::symbol_graph_gen::SymbolGraphOptions::new());
    let_cxx!(mut source_manager = swift::SourceManager::new());
    let_cxx!(mut diagnostic_engine = swift::DiagnosticEngine::from(source_manager.as_mut()));

    fn pre_module_import_callback(_module_name: cxx_llvm::llvm::StringRef<'_>, _is_overlay: bool) -> bool {
        true
    }
    let mut ast_context = swift::AstContext::get(
        lang_options.as_mut(),
        type_checker_options.as_mut(),
        sil_options.as_mut(),
        search_path_options.as_mut(),
        clang_importer_options.as_mut(),
        symbol_graph_options.as_mut(),
        diagnostic_engine.as_mut(),
        pre_module_import_callback,
    );

    let mut clang_importer = {
        let swift_pch_hash = None;
        let dependency_tracker = None;
        swift::ClangImporter::create(ast_context.pin_mut(), swift_pch_hash, dependency_tracker)
    };

    let pch = cache.join("bridging.h.pch");
    std::fs::File::create(&pch)?;

    // Emit a bridging PCH and check that we can read the PCH.
    assert!(!clang_importer.pin_mut().can_read_pch(&pch));
    assert!(!clang_importer.pin_mut().emit_bridging_pch(&bridging_dot_h, &pch));
    assert!(clang_importer.pin_mut().can_read_pch(&pch));

    // Overwrite the PCH with garbage.  We should still be able to read it from the in-memory cache.
    std::fs::write(&pch, "garbage")?;
    assert!(clang_importer.pin_mut().can_read_pch(&pch));

    temp.close()?;
    Ok(())
}
