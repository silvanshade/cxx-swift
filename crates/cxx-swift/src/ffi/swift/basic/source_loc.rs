use cxx_memory::cxx;

pub use crate::abi::swift::basic::source_loc::SourceLoc;

impl SourceLoc {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = SourceLoc> {
        Self::default_new()
    }
}

impl Default for SourceLoc {
    #[inline]
    fn default() -> Self {
        *cxx!(Self::new())
    }
}
