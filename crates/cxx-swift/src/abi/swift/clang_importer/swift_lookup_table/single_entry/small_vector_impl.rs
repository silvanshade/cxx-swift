#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct SmallVectorImpl<'ctx> {
    _layout: [u8; 16],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for SmallVectorImpl<'ctx> {
    type Id = ::cxx::type_id!(
        "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::SmallVectorImpl"
    );
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for SmallVectorImpl<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for SmallVectorImpl<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SmallVectorImpl").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorImpl.hxx");
        #[cxx_name = "SmallVectorImpl"]
        #[allow(unused)]
        type SmallVectorImpl<'ctx> = super::SmallVectorImpl<'ctx>;
        unsafe fn cxx_destruct<'ctx>(This: *mut SmallVectorImpl<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<SmallVectorImpl<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<SmallVectorImpl<'static>>(), 16)
        }
    }
}
