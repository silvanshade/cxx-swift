pub use crate::abi::swift::basic::source_manager::SourceManager;

impl SourceManager {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = SourceManager> {
        Self::default_new()
    }
}
