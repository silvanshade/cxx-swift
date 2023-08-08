pub use crate::auto::swift::clang_importer::effective_clang_context::EffectiveClangContext;

impl EffectiveClangContext {
    #[inline]
    pub fn new() -> impl moveref::New<Output = EffectiveClangContext> {
        Self::default_new()
    }
}

impl Default for EffectiveClangContext {
    #[inline]
    fn default() -> Self {
        *moveref::expr!(Self::new())
    }
}
