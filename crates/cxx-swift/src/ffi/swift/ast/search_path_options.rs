pub use crate::abi::swift::ast::search_path_options::SearchPathOptions;

impl SearchPathOptions {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = SearchPathOptions> {
        Self::default_new()
    }
}
