pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;

use crate::{
    ffi::swift::clang_importer::serialized_swift_name::{
        small_vector::SmallVector as SerializedSwiftNameSmallVector,
        small_vector_impl::SmallVectorImpl as SerializedSwiftNameSmallVectorImpl,
    },
    gen::swift::clang_importer::serialized_swift_name,
};
use core::pin::Pin;
use cxx_llvm::llvm::adt::{
    small_vector::{small_vector_element, SmallVector, SmallVectorElement},
    small_vector_impl::{SmallVectorImpl, SmallVectorImplElement},
    string_ref::StringRef,
};

pub use crate::auto::swift::clang_importer::serialized_swift_name::SerializedSwiftName;

impl<'ctx> SerializedSwiftName<'ctx> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SerializedSwiftName<'ctx>> {
        Self::default_new()
    }

    #[inline]
    pub fn get_name(&self) -> StringRef<'ctx> {
        serialized_swift_name::get_name(self)
    }
}

impl<'ctx> Default for SerializedSwiftName<'ctx> {
    #[inline]
    fn default() -> Self {
        *moveref::expr!(Self::new())
    }
}

impl<'ctx> SmallVectorElement for SerializedSwiftName<'ctx> {
    type DefaultType = SmallVector<Self, { <SerializedSwiftName as SmallVectorElement>::DEFAULT_CAPACITY }>;
    type ReprType = SerializedSwiftNameSmallVector<'ctx>;
    type SizeType = u32;

    const DEFAULT_CAPACITY: usize = small_vector_element::capacity::<Self>();

    #[inline]
    unsafe fn cxx_default_new(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::serialized_swift_name::small_vector::ffi::cxx_default_new(this)
    }

    #[inline]
    unsafe fn cxx_destruct(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::serialized_swift_name::small_vector::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_ref_small_vector_impl(this: &Self::DefaultType) -> &SmallVectorImpl<Self> {
        let this = <Self as SmallVectorElement>::into_repr_ref(this);
        let this =
            crate::gen::swift::clang_importer::serialized_swift_name::small_vector::as_ref_small_vector_impl(this);
        <Self as SmallVectorImplElement>::from_repr_ref(this)
    }

    #[inline]
    fn as_pin_small_vector_impl(this: Pin<&mut Self::DefaultType>) -> Pin<&mut SmallVectorImpl<Self>> {
        let this = <Self as SmallVectorElement>::into_repr_pin(this);
        let this =
            crate::gen::swift::clang_importer::serialized_swift_name::small_vector::as_pin_small_vector_impl(this);
        <Self as SmallVectorImplElement>::from_repr_pin(this)
    }
}

impl<'ctx> SmallVectorImplElement for SerializedSwiftName<'ctx> {
    type ReprType = SerializedSwiftNameSmallVectorImpl<'ctx>;

    #[inline]
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>) {
        let this = <Self as SmallVectorImplElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::serialized_swift_name::small_vector_impl::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self] {
        let this = <Self as SmallVectorImplElement>::into_repr_ref(this);
        crate::gen::swift::clang_importer::serialized_swift_name::small_vector_impl::as_slice(this)
    }

    #[inline]
    unsafe fn as_mut_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> &mut [Self] {
        let this = <Self as SmallVectorImplElement>::into_repr_pin(this);
        crate::gen::swift::clang_importer::serialized_swift_name::small_vector_impl::as_mut_slice(this)
    }
}

#[cfg(test)]
mod test {
    use super::SerializedSwiftName;
    use crate::auto::swift::clang_importer::serialized_swift_name;
    use cxx_llvm::llvm::adt::{small_vector::SmallVectorElement, small_vector_impl::SmallVectorImpl};

    static_assertions::const_assert_eq!(
        core::mem::size_of::<<SerializedSwiftName as SmallVectorElement>::DefaultType>(),
        core::mem::size_of::<serialized_swift_name::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<<SerializedSwiftName as SmallVectorElement>::DefaultType>(),
        core::mem::align_of::<serialized_swift_name::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::size_of::<SmallVectorImpl<SerializedSwiftName>>(),
        core::mem::size_of::<serialized_swift_name::small_vector_impl::SmallVectorImpl>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<SmallVectorImpl<SerializedSwiftName>>(),
        core::mem::align_of::<serialized_swift_name::small_vector_impl::SmallVectorImpl>(),
    );
}
