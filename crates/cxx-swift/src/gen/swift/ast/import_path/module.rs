pub(crate) mod builder;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/AST/ImportPath/Module.hxx");

        // #[namespace = "cxx_swift::swift::ast::import_path::module"]
        // type Module<'ctx> = crate::ffi::swift::ast::import_path::module::Module<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::import_path::module"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
