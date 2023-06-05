#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct ModuleDecl<'ctx> {
    _layout: [u8; 1280],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for ModuleDecl<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::module_decl::ModuleDecl");
    type Kind = ::cxx::kind::Opaque;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for ModuleDecl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ModuleDecl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::module_decl"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/ModuleDecl.hxx");
        #[cxx_name = "ModuleDecl"]
        #[allow(unused)]
        type ModuleDecl<'ctx> = super::ModuleDecl<'ctx>;
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ModuleDecl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ModuleDecl<'static>>(), 1280)
        }
    }
}
