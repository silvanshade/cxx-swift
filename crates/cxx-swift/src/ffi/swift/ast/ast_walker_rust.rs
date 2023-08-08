use crate::{
    ffi::swift::ast::ast_walker::{
        vtable::{AstWalkerConcrete, AstWalkerDyn},
        AstWalker,
    },
    gen::swift::ast::ast_walker_rust,
};
use core::pin::Pin;

#[repr(C, align(8))]
pub struct AstWalkerRust<'state, 'ctx: 'state> {
    _layout: [u8; 32],
    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
    _pinned: core::marker::PhantomPinned,
    _lifetime: core::marker::PhantomData<(&'state (), &'ctx ())>,
}

impl<'state, 'ctx: 'state> core::ops::Deref for AstWalkerRust<'state, 'ctx> {
    type Target = AstWalker<'ctx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        ast_walker_rust::deref(self)
    }
}

impl<'state, 'ctx: 'state> AstWalkerRust<'state, 'ctx> {
    #[inline]
    pub fn deref_pin(self: Pin<&mut Self>) -> Pin<&mut AstWalker<'ctx>> {
        ast_walker_rust::deref_pin(self)
    }
}

impl<'state, 'ctx: 'state> AstWalkerRust<'state, 'ctx> {
    #[inline]
    pub unsafe fn new(
        inner: Pin<&'state mut dyn AstWalkerConcrete<'ctx>>,
    ) -> impl moveref::New<Output = AstWalkerRust<'state, 'ctx>> {
        moveref::new::by_raw(move |this| {
            let this = this.get_unchecked_mut().as_mut_ptr();
            let ast_walker_dyn = Box::new(AstWalkerDyn { inner });
            ast_walker_rust::cxx_placement_new(this, ast_walker_dyn)
        })
    }
}
