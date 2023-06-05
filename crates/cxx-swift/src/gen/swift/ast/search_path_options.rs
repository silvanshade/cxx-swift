#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/SearchPathOptions.hxx");

        // #[namespace = "cxx_swift::swift::ast::search_path_options"]
        // type SearchPathOptions = crate::ffi::swift::ast::search_path_options::SearchPathOptions;
    }

    #[namespace = "cxx_swift::swift::ast::search_path_options"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
