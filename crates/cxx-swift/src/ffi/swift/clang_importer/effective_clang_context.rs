use cxx_memory::cxx;

pub use crate::abi::swift::clang_importer::effective_clang_context::EffectiveClangContext;

impl EffectiveClangContext {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = EffectiveClangContext> {
        Self::default_new()
    }
}

impl Default for EffectiveClangContext {
    #[inline]
    fn default() -> Self {
        *cxx!(Self::new())
    }
}
