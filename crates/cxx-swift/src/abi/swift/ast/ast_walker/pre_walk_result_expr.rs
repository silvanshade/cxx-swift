#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct PreWalkResultExpr<'ctx> {
    _layout: [u8; 24],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for PreWalkResultExpr<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::ast_walker::pre_walk_result_expr::PreWalkResultExpr");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for PreWalkResultExpr<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PreWalkResultExpr").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::ast_walker::pre_walk_result_expr"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ASTWalker/PreWalkResultExpr.hxx");
        #[cxx_name = "PreWalkResultExpr"]
        #[allow(unused)]
        type PreWalkResultExpr<'ctx> = super::PreWalkResultExpr<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<PreWalkResultExpr<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<PreWalkResultExpr<'static>>(), 24)
        }
        :: static_assertions :: assert_impl_all ! (PreWalkResultExpr < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (PreWalkResultExpr < 'static > : :: core :: marker :: Unpin);
    }
}
