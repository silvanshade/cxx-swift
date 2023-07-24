use crate::common::*;
use cxx_clang::clang;
use cxx_llvm::llvm;
use cxx_swift::swift;
use indoc::indoc;

#[test]
fn test() -> BoxResult<()> {
    let temp = tempfile::tempdir()?;

    let cache = temp.path().join("cache");
    std::fs::create_dir(&cache)?;

    let include = temp.path().join("include");
    std::fs::create_dir(&include)?;

    let a_dot_h = include.join("A.h");
    std::fs::write(&a_dot_h, indoc! {r#"
        // Enums and Options
        #define __CF_ENUM_GET_MACRO(_1, _2, NAME, ...) NAME
        #if (__cplusplus && __cplusplus >= 201103L &&                                  \
            (__has_extension(cxx_strong_enums) || __has_feature(objc_fixed_enum))) || \
            (!__cplusplus && __has_feature(objc_fixed_enum))
        #define __CF_NAMED_ENUM(_type, _name)                                          \
        enum _name : _type _name;                                                    \
        enum _name : _type
        #define __CF_ANON_ENUM(_type) enum : _type
        #if (__cplusplus)
        #define CF_OPTIONS(_type, _name)                                               \
        _type _name;                                                                 \
        enum : _type
        #else
        #define CF_OPTIONS(_type, _name)                                               \
        enum _name : _type _name;                                                    \
        enum _name : _type
        #endif
        #else
        #define __CF_NAMED_ENUM(_type, _name)                                          \
        _type _name;                                                                 \
        enum
        #define __CF_ANON_ENUM(_type) enum
        #define CF_OPTIONS(_type, _name)                                               \
        _type _name;                                                                 \
        enum
        #endif

        #define CF_ENUM(...)                                                           \
        __CF_ENUM_GET_MACRO(__VA_ARGS__, __CF_NAMED_ENUM, __CF_ANON_ENUM)(__VA_ARGS__)

        #define NS_ENUM(...) CF_ENUM(__VA_ARGS__)
        #define NS_OPTIONS(_type, _name) CF_OPTIONS(_type, _name)

        #define MY_ERROR_ENUM(_type, _name, _domain)                                   \
        enum _name : _type _name;                                                    \
        enum __attribute__((ns_error_domain(_domain))) _name : _type

        @class NSString;
        extern NSString *const TestErrorDomain;
        typedef MY_ERROR_ENUM(int, TestError, TestErrorDomain){
          TENone,
          TEOne,
          TETwo,
        };

        extern NSString *const ExhaustiveErrorDomain;
        typedef MY_ERROR_ENUM(int, ExhaustiveError, ExhaustiveErrorDomain) {
        EENone, EEOne, EETwo,
        }
        __attribute__((enum_extensibility(closed)));

        extern NSString *const OtherErrorDomain;
        typedef MY_ERROR_ENUM(int, OtherErrorCode, OtherErrorDomain){
          OtherA,
          OtherB,
          OtherC,
        };

        extern NSString *TypedefOnlyErrorDomain;
        typedef enum __attribute__((ns_error_domain(TypedefOnlyErrorDomain))) {
          TypedefOnlyErrorBadness
        } TypedefOnlyError;

        TestError getErr(void);
        ExhaustiveError getExhaustiveErr(void);

        CF_ENUM(short){Zero, One, Two};

        typedef CF_ENUM(unsigned, Color) { Red, Green, Blue };
        Color getColor();

        typedef NS_ENUM(unsigned char, MoreColor) { Cyan, Magenta, Yellow, Black };
        MoreColor getMoreColor();

        typedef CF_OPTIONS(unsigned, ColorOptions) {
          None = 0x0,
          Pastel = 0x1,
          Swift = 0x2
        };
        ColorOptions getColorOptions();

        int foo();

        int baz();

        int qux();

        struct s {
          int n;
        };

        @interface TheClass
        - (TheClass *)initWithSomeDatumX:(int)x andSomeDatumY:(int)y;
        @end

        @protocol TheProtocol
        - (void)doSomethingWithTheClass:(TheClass *)someInstance;
        @end

        #define SQUARE(n) (n * n)
    "#})?;

    let b_dot_h = include.join("B.h");
    std::fs::write(&b_dot_h, indoc! {r#"
        int
        bar();
    "#})?;

    let module_dot_modulemap = include.join("module.modulemap");
    std::fs::write(&module_dot_modulemap, indoc! {r#"
        module M {
            header "A.h"
        }
        module N {
            header "B.h"
        }
    "#})?;

    let bridging_dot_h = temp.path().join("bridging.h");
    std::fs::write(&bridging_dot_h, indoc! {r#"
        #import <A.h>
        #import <B.h>
    "#})?;

    swift::initialize_llvm();

    let_cxx!(mut clang_importer_options = swift::ClangImporterOptions::default());
    clang_importer_options.as_mut().set_bridging_header(&bridging_dot_h);
    clang_importer_options
        .as_mut()
        .modify_extra_args_push_back("-nosysteminc");
    clang_importer_options.as_mut().modify_extra_args_push_back(&format!(
        "-I{}",
        include
            .as_os_str()
            .to_str()
            .expect("path should be a valid UTF-8 string")
    ));
    clang_importer_options.as_mut().set_module_cache_path(&cache);
    clang_importer_options
        .as_mut()
        .set_precompiled_header_output_dir(&cache);

    let_cxx!(mut lang_options = swift::LangOptions::default());
    {
        let_cxx!(target = llvm::Triple::from(("x86_64", "apple", "darwin")));
        lang_options.as_mut().set_target(target)?;
    }
    let_cxx!(mut sil_options = swift::SilOptions::default());
    let_cxx!(mut type_checker_options = swift::TypeCheckerOptions::default());
    let_cxx!(mut search_path_options = swift::SearchPathOptions::default());
    let_cxx!(mut symbol_graph_options = swift::symbol_graph_gen::SymbolGraphOptions::default());
    let_cxx!(mut source_manager = swift::SourceManager::default());
    let_cxx!(mut diagnostic_engine = swift::DiagnosticEngine::from(source_manager.as_mut()));
    let mut ast_context = {
        fn pre_module_import_callback(_module_name: llvm::StringRef, _is_overlay: bool) -> bool {
            true
        }
        swift::AstContext::get(
            lang_options.as_mut(),
            type_checker_options.as_mut(),
            sil_options.as_mut(),
            search_path_options.as_mut(),
            clang_importer_options.as_mut(),
            symbol_graph_options.as_mut(),
            diagnostic_engine.as_mut(),
            pre_module_import_callback,
        )
    };

    let mut clang_importer = {
        let swift_pch_hash = None;
        let dependency_tracker = None;
        swift::ClangImporter::create(ast_context.pin_mut(), swift_pch_hash, dependency_tracker)
    };

    let_cxx!(mut module_names = llvm::SmallVectorImpl::<swift::Identifier>::default());
    let mut module_names = module_names.as_mut().as_pin();
    clang_importer
        .pin_mut()
        .collect_visible_top_level_module_names(module_names.as_mut());

    let mut module_count = 0;

    for module_name in module_names.iter().copied() {
        println!("swift module: processing");
        module_count += 1;
        if let Some(module_decl) = {
            let source_loc = *cxx!(swift::SourceLoc::default());
            let module_path = {
                let_cxx!(module_path_builder = swift::ast::import_path::module::Builder::from(module_name));
                module_path_builder.get()
            };
            let allow_memory_cache = None;
            clang_importer
                .pin_mut()
                .load_module(source_loc, module_path, allow_memory_cache)
        } {
            println!("swift module: successfully loaded");
            if let Some(clang_module) = module_decl.find_underlying_clang_module() {
                println!("swift module: found underlying clang module");
                if let Some(mut table) = clang_importer.find_lookup_table_for_module(clang_module) {
                    println!("swift module: successfully loaded swift lookup table for clang module");
                    table.as_mut().deserialize_all();
                    table.as_mut().dump();
                    let search_context = *cxx!(swift::EffectiveClangContext::default());
                    let_cxx!(base_names = table.all_base_names());
                    println!("swift module: processing base names from lookup table\n");
                    for base_name in base_names.iter().copied() {
                        println!("name: {}", base_name.get_name().as_str()?);
                        let_cxx!(entries = table.lookup(base_name, search_context));
                        for entry in &*entries {
                            if let Some(named_decl) = entry.cast_as_named_decl() {
                                println!("entry: <NamedDecl>");
                                let kind = named_decl.get_kind();
                                println!("kind: {kind:#?}");
                                match kind {
                                    clang::DeclKind::ObjCInterface => {
                                        if let Some(_objc_interface_decl) = named_decl.cast_as_objc_interface_decl() {
                                            println!("<successfully casted to ObjCInterfaceDecl>");
                                        }
                                    },
                                    clang::DeclKind::ObjCProtocol => {
                                        if let Some(_objc_protocol_decl) = named_decl.cast_as_objc_protocol_decl() {
                                            println!("<successfully casted to ObjCProtocolDecl>");
                                        }
                                    },
                                    clang::DeclKind::ObjCMethod => {
                                        if let Some(_objc_method_decl) = named_decl.cast_as_objc_method_decl() {
                                            println!("<successfully casted to ObjCMethodDecl>");
                                        }
                                    },
                                    clang::DeclKind::Record => {
                                        if let Some(_record_decl) = named_decl.cast_as_record_decl() {
                                            println!("<successfully casted to RecordDecl>");
                                        }
                                    },
                                    clang::DeclKind::Typedef => {
                                        if let Some(_typedef_decl) = named_decl.cast_as_typedef_decl() {
                                            println!("<successfully casted to TypedefDecl>");
                                        }
                                    },
                                    clang::DeclKind::Field => {
                                        if let Some(_field_decl) = named_decl.cast_as_field_decl() {
                                            println!("<successfully casted to FieldDecl>");
                                        }
                                    },
                                    clang::DeclKind::Function => {
                                        if let Some(_function_decl) = named_decl.cast_as_function_decl() {
                                            println!("<successfully casted to FunctionDecl>");
                                        }
                                    },
                                    _ => {},
                                }
                                println!();
                                continue;
                            }
                            if let Some(_macro_info) = entry.cast_as_macro_info() {
                                println!("entry: <MacroInfo>\n");
                                continue;
                            }
                            if let Some(_module_macro) = entry.cast_as_module_macro() {
                                println!("entry: <ModuleMacro>\n");
                                continue;
                            }
                        }
                    }
                } else {
                    println!("swift module: failed to load swift lookup table for clang module");
                }
            }
        } else {
            println!("swift module: failed to load");
        }
        println!();
    }

    assert_eq!(module_count, 2);

    temp.close()?;
    Ok(())
}
