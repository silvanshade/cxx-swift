#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub(crate) mod parent_ty;
pub(crate) mod pre_walk_result_expr;
#[repr(C, align(8))]
pub struct AstWalker<'ctx> {
    _layout: [u8; 24],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for AstWalker<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_walker::ASTWalker");
    type Kind = ::cxx::kind::Opaque;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for AstWalker<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AstWalker").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::ast_walker"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker.hxx");
        #[cxx_name = "ASTWalker"]
        #[allow(unused)]
        type AstWalker<'ctx> = super::AstWalker<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<AstWalker<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<AstWalker<'static>>(), 24)
        }
    }
}
