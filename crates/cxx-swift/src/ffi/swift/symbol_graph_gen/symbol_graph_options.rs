pub use crate::abi::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;

impl SymbolGraphOptions {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = SymbolGraphOptions> {
        Self::default_new()
    }
}
