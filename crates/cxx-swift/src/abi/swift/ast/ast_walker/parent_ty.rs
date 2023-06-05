#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct ParentTy<'ctx> {
    _layout: [u8; 16],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for ParentTy<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_walker::parent_ty::ParentTy");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for ParentTy<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ParentTy").finish()
    }
}
impl<'ctx> ParentTy<'ctx> {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = ParentTy<'ctx>> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl<'ctx> ::cxx_memory::CopyNew for ParentTy<'ctx> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for ParentTy<'ctx> {
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
    #[namespace = "cxx_swift::swift::ast::ast_walker::parent_ty"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/ParentTy.hxx");
        #[cxx_name = "ParentTy"]
        #[allow(unused)]
        type ParentTy<'ctx> = super::ParentTy<'ctx>;
        unsafe fn cxx_copy_new<'ctx>(This: *mut ParentTy<'ctx>, that: &ParentTy<'ctx>);
        unsafe fn cxx_move_new<'ctx>(This: *mut ParentTy<'ctx>, that: *mut ParentTy<'ctx>);
        unsafe fn cxx_default_new<'ctx>(This: *mut ParentTy<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ParentTy<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ParentTy<'static>>(), 16)
        }
        :: static_assertions :: assert_impl_all ! (ParentTy < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (ParentTy < 'static > : :: core :: marker :: Unpin);
    }
}
