pub use crate::abi::swift::ast::identifier::small_vector::SmallVector;

impl<'ctx> SmallVector<'ctx> {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = SmallVector<'ctx>> {
        Self::default_new()
    }
}
