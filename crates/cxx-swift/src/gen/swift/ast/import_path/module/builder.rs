#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Identifier.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ImportPath/Module.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/ImportPath/Module/Builder.hxx");

        #[namespace = "cxx_swift::swift::ast::import_path::module::builder"]
        type Builder<'ctx> = crate::ffi::swift::ast::import_path::module::builder::Builder<'ctx>;

        #[namespace = "cxx_swift::swift::ast::identifier"]
        type Identifier<'ctx> = crate::ffi::swift::ast::identifier::Identifier<'ctx>;

        #[namespace = "cxx_swift::swift::ast::import_path::module"]
        #[cxx_name = "Module"]
        type ImportPathModule<'ctx> = crate::ffi::swift::ast::import_path::module::Module<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::import_path::module::builder"]
    unsafe extern "C++" {
        unsafe fn cxx_placement_new_from_identifier<'ctx>(This: *mut Builder<'ctx>, name: Identifier<'ctx>);

        fn get<'ctx>(This: &Builder<'ctx>) -> ImportPathModule<'ctx>;
    }
}
pub(crate) use self::ffi::*;
