#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct SwiftLookupTableSingleEntry<'ctx> {
    _layout: [u8; 8],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for SwiftLookupTableSingleEntry<'ctx> {
    type Id = ::cxx::type_id!(
        "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry"
    );
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for SwiftLookupTableSingleEntry<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SwiftLookupTableSingleEntry").finish()
    }
}
impl<'ctx> SwiftLookupTableSingleEntry<'ctx> {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = SwiftLookupTableSingleEntry<'ctx>> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl<'ctx> ::cxx_memory::CopyNew for SwiftLookupTableSingleEntry<'ctx> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for SwiftLookupTableSingleEntry<'ctx> {
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
    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry.hxx");
        #[cxx_name = "SwiftLookupTableSingleEntry"]
        #[allow(unused)]
        type SwiftLookupTableSingleEntry<'ctx> = super::SwiftLookupTableSingleEntry<'ctx>;
        unsafe fn cxx_copy_new<'ctx>(
            This: *mut SwiftLookupTableSingleEntry<'ctx>,
            that: &SwiftLookupTableSingleEntry<'ctx>,
        );
        unsafe fn cxx_move_new<'ctx>(
            This: *mut SwiftLookupTableSingleEntry<'ctx>,
            that: *mut SwiftLookupTableSingleEntry<'ctx>,
        );
        unsafe fn cxx_default_new<'ctx>(This: *mut SwiftLookupTableSingleEntry<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<SwiftLookupTableSingleEntry<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<SwiftLookupTableSingleEntry<'static>>(), 8)
        }
        :: static_assertions :: assert_impl_all ! (SwiftLookupTableSingleEntry < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (SwiftLookupTableSingleEntry < 'static > : :: core :: marker :: Unpin);
    }
}
