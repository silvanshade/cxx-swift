use cxx_memory::cxx;

pub use crate::abi::swift::ast::ast_walker::parent_ty::ParentTy;

impl<'ctx> ParentTy<'ctx> {
    #[inline]
    pub fn new() -> impl cxx_memory::New<Output = ParentTy<'ctx>> {
        Self::default_new()
    }
}

impl<'ctx> Default for ParentTy<'ctx> {
    #[inline]
    fn default() -> Self {
        *cxx!(Self::new())
    }
}
