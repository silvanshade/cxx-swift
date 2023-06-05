#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct AstWalkerBase<'ctx> {
    _layout: [u8; 24],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for AstWalkerBase<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_walker_base::ASTWalkerBase");
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for AstWalkerBase<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for AstWalkerBase<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AstWalkerBase").finish()
    }
}
impl<'ctx> AstWalkerBase<'ctx> {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = AstWalkerBase<'ctx>> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for AstWalkerBase<'ctx> {
    #[inline]
    unsafe fn move_new(
        that: ::core::pin::Pin<::cxx_memory::MoveRef<'_, Self>>,
        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
    ) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
        self::ffi::cxx_move_new(this, that)
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::ast_walker_base"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalkerBase.hxx");
        #[cxx_name = "ASTWalkerBase"]
        #[allow(unused)]
        type AstWalkerBase<'ctx> = super::AstWalkerBase<'ctx>;
        unsafe fn cxx_move_new<'ctx>(This: *mut AstWalkerBase<'ctx>, that: *mut AstWalkerBase<'ctx>);
        unsafe fn cxx_default_new<'ctx>(This: *mut AstWalkerBase<'ctx>);
        unsafe fn cxx_destruct<'ctx>(This: *mut AstWalkerBase<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<AstWalkerBase<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<AstWalkerBase<'static>>(), 24)
        }
    }
}
