pub use crate::abi::swift::basic::type_checker_options::TypeCheckerOptions;

impl TypeCheckerOptions {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = TypeCheckerOptions> {
        Self::default_new()
    }
}
