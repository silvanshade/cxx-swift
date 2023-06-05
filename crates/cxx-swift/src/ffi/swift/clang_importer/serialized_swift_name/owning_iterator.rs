use crate::{
    ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName,
    gen::swift::clang_importer::serialized_swift_name::owning_iterator,
};
use core::pin::Pin;

pub use crate::abi::swift::clang_importer::serialized_swift_name::owning_iterator::SerializedSwiftNameOwningIterator;

impl<'ctx> SerializedSwiftNameOwningIterator<'ctx> {
    #[inline]
    pub fn iter(self: Pin<&mut Self>) -> impl Iterator<Item = &SerializedSwiftName<'ctx>> {
        Iter { inner: self }
    }

    #[inline]
    pub fn iter_pin(self: Pin<&mut Self>) -> impl Iterator<Item = Pin<&mut SerializedSwiftName<'ctx>>> {
        IterPin { inner: self }
    }
}

struct Iter<'a, 'ctx: 'a> {
    inner: Pin<&'a mut SerializedSwiftNameOwningIterator<'ctx>>,
}

impl<'a, 'ctx: 'a> Iterator for Iter<'a, 'ctx> {
    type Item = &'a SerializedSwiftName<'ctx>;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = owning_iterator::next(self.inner.as_mut());
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}

struct IterPin<'a, 'ctx: 'a> {
    inner: Pin<&'a mut SerializedSwiftNameOwningIterator<'ctx>>,
}

impl<'a, 'ctx: 'a> Iterator for IterPin<'a, 'ctx> {
    type Item = Pin<&'a mut SerializedSwiftName<'ctx>>;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = owning_iterator::next(self.inner.as_mut());
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Pin::new_unchecked(&mut *ptr) })
        }
    }
}
