pub use crate::auto::swift::ast::search_path_options::SearchPathOptions;

impl SearchPathOptions {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SearchPathOptions> {
        Self::default_new()
    }
}
