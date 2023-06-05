#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct AstWalkerRust<'state, 'ctx: 'state> {
    _layout: [u8; 32],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'state (), &'ctx ())>,
}
unsafe impl<'state, 'ctx: 'state> ::cxx::ExternType for AstWalkerRust<'state, 'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_walker_rust::ASTWalkerRust");
    type Kind = ::cxx::kind::Opaque;
}
impl<'state, 'ctx: 'state> ::core::ops::Drop for AstWalkerRust<'state, 'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'state, 'ctx: 'state> ::core::fmt::Debug for AstWalkerRust<'state, 'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AstWalkerRust").finish()
    }
}
unsafe impl<'state, 'ctx: 'state> ::cxx_memory::MoveNew for AstWalkerRust<'state, 'ctx> {
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
    #[namespace = "cxx_swift::swift::ast::ast_walker_rust"]
    unsafe extern "C++" {
        include!("cxx-swift/cxx/include/swift/AST/ASTWalkerRust.hxx");
        #[cxx_name = "ASTWalkerRust"]
        #[allow(unused)]
        type AstWalkerRust<'state, 'ctx> = super::AstWalkerRust<'state, 'ctx>;
        unsafe fn cxx_move_new<'state, 'ctx>(
            This: *mut AstWalkerRust<'state, 'ctx>,
            that: *mut AstWalkerRust<'state, 'ctx>,
        );
        unsafe fn cxx_destruct<'state, 'ctx>(This: *mut AstWalkerRust<'state, 'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<AstWalkerRust<'static, 'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<AstWalkerRust<'static, 'static>>(), 32)
        }
    }
}
