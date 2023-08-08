pub use crate::auto::swift::basic::type_checker_options::TypeCheckerOptions;

impl TypeCheckerOptions {
    #[inline]
    pub fn new() -> impl moveref::New<Output = TypeCheckerOptions> {
        Self::default_new()
    }
}
