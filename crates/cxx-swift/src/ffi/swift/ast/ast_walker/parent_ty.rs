pub use crate::auto::swift::ast::ast_walker::parent_ty::ParentTy;

impl<'ctx> ParentTy<'ctx> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = ParentTy<'ctx>> {
        Self::default_new()
    }
}

impl<'ctx> Default for ParentTy<'ctx> {
    #[inline]
    fn default() -> Self {
        *moveref::expr!(Self::new())
    }
}
