#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct SwiftLookupTableSingleEntryOwningIterator<'ctx> {
    _layout: [u8; 32],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for SwiftLookupTableSingleEntryOwningIterator<'ctx> {
    type Id = :: cxx :: type_id ! ("cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator::SwiftLookupTableSingleEntryOwningIterator") ;
    type Kind = ::cxx::kind::Opaque;
}
impl<'ctx> ::core::ops::Drop for SwiftLookupTableSingleEntryOwningIterator<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for SwiftLookupTableSingleEntryOwningIterator<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SwiftLookupTableSingleEntryOwningIterator").finish()
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for SwiftLookupTableSingleEntryOwningIterator<'ctx> {
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
    #[namespace = "cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/OwningIterator.hxx");
        #[cxx_name = "SwiftLookupTableSingleEntryOwningIterator"]
        #[allow(unused)]
        type SwiftLookupTableSingleEntryOwningIterator<'ctx> = super::SwiftLookupTableSingleEntryOwningIterator<'ctx>;
        unsafe fn cxx_move_new<'ctx>(
            This: *mut SwiftLookupTableSingleEntryOwningIterator<'ctx>,
            that: *mut SwiftLookupTableSingleEntryOwningIterator<'ctx>,
        );
        unsafe fn cxx_destruct<'ctx>(This: *mut SwiftLookupTableSingleEntryOwningIterator<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(
                ::core::mem::align_of::<SwiftLookupTableSingleEntryOwningIterator<'static>>(),
                8
            )
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(
                ::core::mem::size_of::<SwiftLookupTableSingleEntryOwningIterator<'static>>(),
                32
            )
        }
    }
}
