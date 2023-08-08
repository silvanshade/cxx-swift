pub use crate::auto::swift::ast::identifier::small_vector::SmallVector;

impl<'ctx> SmallVector<'ctx> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SmallVector<'ctx>> {
        Self::default_new()
    }
}
