pub use crate::auto::swift::clang_importer::serialized_swift_name::small_vector::SmallVector;

impl<'ctx> SmallVector<'ctx> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SmallVector<'ctx>> {
        Self::default_new()
    }
}
