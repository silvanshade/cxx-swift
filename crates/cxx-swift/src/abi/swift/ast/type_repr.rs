#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct TypeRepr<'ctx> {
    _layout: [u8; 8],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for TypeRepr<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::type_repr::TypeRepr");
    type Kind = ::cxx::kind::Opaque;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for TypeRepr<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypeRepr").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::type_repr"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/TypeRepr.hxx");
        #[cxx_name = "TypeRepr"]
        #[allow(unused)]
        type TypeRepr<'ctx> = super::TypeRepr<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<TypeRepr<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<TypeRepr<'static>>(), 8)
        }
    }
}
