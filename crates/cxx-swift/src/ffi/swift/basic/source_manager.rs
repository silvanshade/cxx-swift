pub use crate::auto::swift::basic::source_manager::SourceManager;

impl SourceManager {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SourceManager> {
        Self::default_new()
    }
}
