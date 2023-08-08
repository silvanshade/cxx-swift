use crate::gen::swift::ast::module_decl;
use cxx_clang::clang::basic::module::Module as ClangModule;

pub use crate::auto::swift::ast::module_decl::ModuleDecl;

impl<'ctx> ModuleDecl<'ctx> {
    pub fn find_underlying_clang_module(&self) -> Option<&ClangModule<'ctx>> {
        let ptr = module_decl::find_underlying_clang_module(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}
