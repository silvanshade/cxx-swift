pub use crate::auto::swift::ast::sil_options::SilOptions;

impl SilOptions {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SilOptions> {
        Self::default_new()
    }
}
