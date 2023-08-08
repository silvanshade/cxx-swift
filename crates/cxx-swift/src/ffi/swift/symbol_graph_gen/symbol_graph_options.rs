pub use crate::auto::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;

impl SymbolGraphOptions {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SymbolGraphOptions> {
        Self::default_new()
    }
}
