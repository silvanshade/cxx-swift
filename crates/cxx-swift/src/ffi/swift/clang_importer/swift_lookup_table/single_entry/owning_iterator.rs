use crate::{
    ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry,
    gen::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator,
};
use core::pin::Pin;

pub use crate::abi::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator::SwiftLookupTableSingleEntryOwningIterator;

impl<'ctx> SwiftLookupTableSingleEntryOwningIterator<'ctx> {
    #[inline]
    pub fn iter(self: Pin<&mut Self>) -> impl Iterator<Item = &SwiftLookupTableSingleEntry<'ctx>> {
        Iter { inner: self }
    }

    #[inline]
    pub fn iter_pin(self: Pin<&mut Self>) -> impl Iterator<Item = Pin<&mut SwiftLookupTableSingleEntry<'ctx>>> {
        IterPin { inner: self }
    }
}

struct Iter<'a, 'ctx: 'a> {
    inner: Pin<&'a mut SwiftLookupTableSingleEntryOwningIterator<'ctx>>,
}

impl<'a, 'ctx: 'a> Iterator for Iter<'a, 'ctx> {
    type Item = &'a SwiftLookupTableSingleEntry<'ctx>;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = owning_iterator::next(self.inner.as_mut());
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}

struct IterPin<'a, 'ctx: 'a> {
    inner: Pin<&'a mut SwiftLookupTableSingleEntryOwningIterator<'ctx>>,
}

impl<'a, 'ctx: 'a> Iterator for IterPin<'a, 'ctx> {
    type Item = Pin<&'a mut SwiftLookupTableSingleEntry<'ctx>>;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = owning_iterator::next(self.inner.as_mut());
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Pin::new_unchecked(&mut *ptr) })
        }
    }
}
