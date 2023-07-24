pub(crate) mod small_vector;
pub(crate) mod small_vector_impl;

use crate::ffi::swift::ast::identifier::{
    small_vector::SmallVector as IdentifierSmallVector,
    small_vector_impl::SmallVectorImpl as IdentifierSmallVectorImpl,
};
use core::pin::Pin;
use cxx_llvm::llvm::adt::{
    small_vector::{small_vector_element, SmallVector, SmallVectorElement},
    small_vector_impl::{SmallVectorImpl, SmallVectorImplElement},
};

pub use crate::abi::swift::ast::identifier::Identifier;

impl<'ctx> SmallVectorElement for Identifier<'ctx> {
    type DefaultType = SmallVector<Self, { <Identifier as SmallVectorElement>::DEFAULT_CAPACITY }>;
    type ReprType = IdentifierSmallVector<'ctx>;
    type SizeType = u32;

    const DEFAULT_CAPACITY: usize = small_vector_element::capacity::<Self>();

    #[inline]
    unsafe fn cxx_default_new(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::abi::swift::ast::identifier::small_vector::ffi::cxx_default_new(this)
    }

    #[inline]
    unsafe fn cxx_destruct(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::abi::swift::ast::identifier::small_vector::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_ref_small_vector_impl(this: &Self::DefaultType) -> &SmallVectorImpl<Self> {
        let this = <Self as SmallVectorElement>::into_repr_ref(this);
        let this = crate::gen::swift::ast::identifier::small_vector::as_ref_small_vector_impl(this);
        <Self as SmallVectorImplElement>::from_repr_ref(this)
    }

    #[inline]
    fn as_pin_small_vector_impl(this: Pin<&mut Self::DefaultType>) -> Pin<&mut SmallVectorImpl<Self>> {
        let this = <Self as SmallVectorElement>::into_repr_pin(this);
        let this = crate::gen::swift::ast::identifier::small_vector::as_pin_small_vector_impl(this);
        <Self as SmallVectorImplElement>::from_repr_pin(this)
    }
}

impl<'ctx> SmallVectorImplElement for Identifier<'ctx> {
    type ReprType = IdentifierSmallVectorImpl<'ctx>;

    #[inline]
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>) {
        let this = <Self as SmallVectorImplElement>::into_repr_mut_ptr(this);
        crate::abi::swift::ast::identifier::small_vector_impl::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self] {
        let this = <Self as SmallVectorImplElement>::into_repr_ref(this);
        crate::gen::swift::ast::identifier::small_vector_impl::as_slice(this)
    }

    #[inline]
    unsafe fn as_mut_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> &mut [Self] {
        let this = <Self as SmallVectorImplElement>::into_repr_pin(this);
        crate::gen::swift::ast::identifier::small_vector_impl::as_mut_slice(this)
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;
    use crate::abi::swift::ast::identifier;
    use cxx_llvm::llvm::adt::{small_vector::SmallVectorElement, small_vector_impl::SmallVectorImpl};

    static_assertions::const_assert_eq!(
        core::mem::size_of::<<Identifier as SmallVectorElement>::DefaultType>(),
        core::mem::size_of::<identifier::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<<Identifier as SmallVectorElement>::DefaultType>(),
        core::mem::align_of::<identifier::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::size_of::<SmallVectorImpl<Identifier>>(),
        core::mem::size_of::<identifier::small_vector_impl::SmallVectorImpl>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<SmallVectorImpl<Identifier>>(),
        core::mem::align_of::<identifier::small_vector_impl::SmallVectorImpl>(),
    );
}
