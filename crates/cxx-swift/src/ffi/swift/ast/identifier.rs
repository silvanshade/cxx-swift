use core::pin::Pin;
use cxx_llvm::llvm::adt::{
    small_vector::{small_vector_element, SmallVector, SmallVectorElement},
    small_vector_impl::{SmallVectorImpl, SmallVectorImplElement},
};

pub use crate::abi::swift::ast::identifier::Identifier;

impl<'ctx> SmallVectorElement for Identifier<'ctx> {
    type DefaultType = SmallVector<Self, { <Identifier as SmallVectorElement>::DEFAULT_CAPACITY }>;
    type SizeType = u32;

    const DEFAULT_CAPACITY: usize = small_vector_element::capacity::<Self>();

    #[inline]
    unsafe fn cxx_default_new(this: *mut Self::DefaultType) {
        let this = this.cast();
        crate::abi::llvm::adt::small_vector::swift::ast::identifier::ffi::cxx_default_new(this)
    }

    #[inline]
    unsafe fn cxx_destruct(this: *mut Self::DefaultType) {
        let this = this.cast();
        crate::abi::llvm::adt::small_vector::swift::ast::identifier::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_ref_small_vector_impl(this: &Self::DefaultType) -> &SmallVectorImpl<Self> {
        let this = unsafe { core::mem::transmute(this) };
        let this = crate::gen::llvm::adt::small_vector::swift::ast::identifier::as_ref_small_vector_impl(this);
        unsafe { core::mem::transmute(this) }
    }

    #[inline]
    fn as_pin_small_vector_impl(this: Pin<&mut Self::DefaultType>) -> Pin<&mut SmallVectorImpl<Self>> {
        let this = unsafe { core::mem::transmute(this) };
        let this = crate::gen::llvm::adt::small_vector::swift::ast::identifier::as_pin_small_vector_impl(this);
        unsafe { core::mem::transmute(this) }
    }
}

impl<'ctx> SmallVectorImplElement for Identifier<'ctx> {
    #[inline]
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>) {
        let this = this.cast();
        crate::abi::llvm::adt::small_vector_impl::swift::ast::identifier::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self] {
        let this = unsafe { core::mem::transmute(this) };
        crate::gen::llvm::adt::small_vector_impl::swift::ast::identifier::as_slice(this)
    }

    #[inline]
    unsafe fn as_mut_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> &mut [Self] {
        let this = unsafe { core::mem::transmute(this) };
        crate::gen::llvm::adt::small_vector_impl::swift::ast::identifier::as_mut_slice(this)
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;
    use crate::abi::llvm::adt::{small_vector, small_vector_impl};
    use cxx_llvm::llvm::adt::{small_vector::SmallVectorElement, small_vector_impl::SmallVectorImpl};

    static_assertions::const_assert_eq!(
        core::mem::size_of::<<Identifier as SmallVectorElement>::DefaultType>(),
        core::mem::size_of::<small_vector::swift::ast::identifier::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<<Identifier as SmallVectorElement>::DefaultType>(),
        core::mem::align_of::<small_vector::swift::ast::identifier::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::size_of::<SmallVectorImpl<Identifier>>(),
        core::mem::size_of::<small_vector_impl::swift::ast::identifier::SmallVectorImpl>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<SmallVectorImpl<Identifier>>(),
        core::mem::align_of::<small_vector_impl::swift::ast::identifier::SmallVectorImpl>(),
    );
}
