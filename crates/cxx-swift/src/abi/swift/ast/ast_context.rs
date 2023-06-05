#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct AstContext<'cfg> {
    _layout: [u8; 3832],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'cfg (),)>,
}
unsafe impl<'cfg> ::cxx::ExternType for AstContext<'cfg> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_context::ASTContext");
    type Kind = ::cxx::kind::Opaque;
}
impl<'cfg> ::core::ops::Drop for AstContext<'cfg> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'cfg> ::core::fmt::Debug for AstContext<'cfg> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AstContext").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::ast_context"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTContext.hxx");
        #[cxx_name = "ASTContext"]
        #[allow(unused)]
        type AstContext<'cfg> = super::AstContext<'cfg>;
        unsafe fn cxx_destruct<'cfg>(This: *mut AstContext<'cfg>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<AstContext<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<AstContext<'static>>(), 3832)
        }
    }
}
