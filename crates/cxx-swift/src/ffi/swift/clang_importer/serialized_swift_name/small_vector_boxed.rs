use crate::{
    ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName,
    gen::swift::clang_importer::serialized_swift_name::small_vector_boxed,
};
use core::pin::Pin;
use cxx_llvm::llvm::{adt::small_vector_impl::SmallVectorImplElement, SmallVectorImpl};

pub use crate::abi::swift::clang_importer::serialized_swift_name::small_vector_boxed::SmallVectorBoxed;

impl<'ctx> SmallVectorBoxed<'ctx> {
    #[inline]
    pub fn as_ref_small_vector_impl(&self) -> &SmallVectorImpl<SerializedSwiftName<'ctx>> {
        let this = small_vector_boxed::as_ref_small_vector_impl(self);
        SerializedSwiftName::from_repr_ref(this)
    }

    #[inline]
    pub fn as_pin_small_vector_impl(self: Pin<&mut Self>) -> Pin<&mut SmallVectorImpl<SerializedSwiftName<'ctx>>> {
        let this = small_vector_boxed::as_pin_small_vector_impl(self);
        SerializedSwiftName::from_repr_pin(this)
    }
}

impl<'ctx> AsRef<SmallVectorImpl<SerializedSwiftName<'ctx>>> for &SmallVectorBoxed<'ctx> {
    #[inline]
    fn as_ref(&self) -> &SmallVectorImpl<SerializedSwiftName<'ctx>> {
        self.as_ref_small_vector_impl()
    }
}

impl<'ctx> core::ops::Deref for SmallVectorBoxed<'ctx> {
    type Target = SmallVectorImpl<SerializedSwiftName<'ctx>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref_small_vector_impl()
    }
}

impl<'a, 'ctx> IntoIterator for &'a SmallVectorBoxed<'ctx> {
    type Item = <&'a SmallVectorImpl<SerializedSwiftName<'ctx>> as IntoIterator>::Item;
    type IntoIter = <&'a SmallVectorImpl<SerializedSwiftName<'ctx>> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref_small_vector_impl().into_iter()
    }
}
