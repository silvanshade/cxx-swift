#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-clang-abi/cxx/include/clang/Basic/Module.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ModuleDecl.hxx");

        #[namespace = "cxx_clang::clang::basic::module"]
        #[cxx_name = "Module"]
        type ClangModule<'ctx> = cxx_clang::clang::basic::module::Module<'ctx>;

        #[namespace = "cxx_swift::swift::ast::module_decl"]
        type ModuleDecl<'ctx> = crate::ffi::swift::ast::module_decl::ModuleDecl<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::module_decl"]
    unsafe extern "C++" {
        fn find_underlying_clang_module<'ctx>(This: &ModuleDecl<'ctx>) -> *const ClangModule<'ctx>;
    }
}
pub(crate) use self::ffi::*;
