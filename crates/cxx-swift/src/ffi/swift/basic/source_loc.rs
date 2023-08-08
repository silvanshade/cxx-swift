pub use crate::auto::swift::basic::source_loc::SourceLoc;

impl SourceLoc {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SourceLoc> {
        Self::default_new()
    }
}

impl Default for SourceLoc {
    #[inline]
    fn default() -> Self {
        *moveref::expr!(Self::new())
    }
}
