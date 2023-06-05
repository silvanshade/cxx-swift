mod common;

mod swift {
    // mod ast {
    //     mod ast_walker;
    // }
    mod clang_importer {
        mod clang_importer {
            mod emit_bridging_pch;
        }
        mod swift_lookup_table {
            mod deserialize_and_dump_from_clang_modules;
        }
    }
}
