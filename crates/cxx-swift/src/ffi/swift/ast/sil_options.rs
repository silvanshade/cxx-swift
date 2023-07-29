pub use crate::abi::swift::ast::sil_options::SilOptions;

impl SilOptions {
    #[inline]
    pub fn new() -> impl ::cxx_memory::New<Output = SilOptions> {
        Self::default_new()
    }
}
