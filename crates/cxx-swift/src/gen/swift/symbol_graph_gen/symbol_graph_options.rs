#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/SymbolGraphGen/SymbolGraphOptions.hxx");

        // #[namespace = "cxx_swift::swift::symbol_graph_gen::symbol_graph_options"]
        // type SymbolGraphOptions = crate::ffi::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;
    }

    #[namespace = "cxx_swift::swift::symbol_graph_gen::symbol_graph_options"]
    unsafe extern "C++" {}
}
// pub(crate) use self::ffi::*;
