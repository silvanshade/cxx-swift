use crate::{
    ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry,
    gen::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed,
};
use core::pin::Pin;
use cxx_llvm::llvm::{adt::small_vector_impl::SmallVectorImplElement, SmallVectorImpl};

pub use crate::abi::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed::SmallVectorBoxed;

impl<'ctx> SmallVectorBoxed<'ctx> {
    #[inline]
    pub fn as_ref_small_vector_impl(&self) -> &SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>> {
        let this = small_vector_boxed::as_ref_small_vector_impl(self);
        SwiftLookupTableSingleEntry::from_repr_ref(this)
    }

    #[inline]
    pub fn as_pin_small_vector_impl(
        self: Pin<&mut Self>,
    ) -> Pin<&mut SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>>> {
        let this = small_vector_boxed::as_pin_small_vector_impl(self);
        SwiftLookupTableSingleEntry::from_repr_pin(this)
    }
}

impl<'ctx> AsRef<SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>>> for &SmallVectorBoxed<'ctx> {
    #[inline]
    fn as_ref(&self) -> &SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>> {
        self.as_ref_small_vector_impl()
    }
}

impl<'ctx> core::ops::Deref for SmallVectorBoxed<'ctx> {
    type Target = SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_small_vector_impl()
    }
}

impl<'a, 'ctx> IntoIterator for &'a SmallVectorBoxed<'ctx> {
    type Item = <&'a SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>> as IntoIterator>::Item;
    type IntoIter = <&'a SmallVectorImpl<SwiftLookupTableSingleEntry<'ctx>> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref_small_vector_impl().into_iter()
    }
}
