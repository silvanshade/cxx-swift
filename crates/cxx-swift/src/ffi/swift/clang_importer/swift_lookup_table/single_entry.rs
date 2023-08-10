pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;

use crate::{
    ffi::swift::clang_importer::swift_lookup_table::single_entry::{
        small_vector::SmallVector as SwiftLookupTableSingleEntrySmallVector,
        small_vector_impl::SmallVectorImpl as SwiftLookupTableSingleEntrySmallVectorImpl,
    },
    gen::swift::clang_importer::swift_lookup_table::single_entry,
};
use core::pin::Pin;
use cxx_clang::clang::{
    ast::decl::named_decl::NamedDecl as ClangNamedDecl,
    lex::macro_info::{module_macro::ModuleMacro as ClangModuleMacro, MacroInfo as ClangMacroInfo},
};
use cxx_llvm::llvm::adt::{
    small_vector::{small_vector_element, SmallVector, SmallVectorElement},
    small_vector_impl::{SmallVectorImpl, SmallVectorImplElement},
};

pub use crate::auto::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry;

impl<'ctx> SwiftLookupTableSingleEntry<'ctx> {
    #[inline]
    pub fn new() -> impl moveref::New<Output = SwiftLookupTableSingleEntry<'ctx>> {
        Self::default_new()
    }

    #[inline]
    pub fn cast_as_named_decl(&self) -> Option<&ClangNamedDecl<'ctx>> {
        let ptr = single_entry::cast_as_named_decl(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_macro_info(&self) -> Option<&ClangMacroInfo<'ctx>> {
        let ptr = single_entry::cast_as_macro_info(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }

    #[inline]
    pub fn cast_as_module_macro(&self) -> Option<&ClangModuleMacro<'ctx>> {
        let ptr = single_entry::cast_as_module_macro(self);
        if ptr.is_null() { None } else { Some(unsafe { &*ptr }) }
    }
}

impl<'ctx> Default for SwiftLookupTableSingleEntry<'ctx> {
    #[inline]
    fn default() -> Self {
        *moveref::expr!(Self::new())
    }
}

impl<'ctx> SmallVectorElement for SwiftLookupTableSingleEntry<'ctx> {
    type DefaultType = SmallVector<Self, { <SwiftLookupTableSingleEntry as SmallVectorElement>::DEFAULT_CAPACITY }>;
    type ReprType = SwiftLookupTableSingleEntrySmallVector<'ctx>;
    type SizeType = u32;

    const DEFAULT_CAPACITY: usize = small_vector_element::capacity::<Self>();

    #[inline]
    unsafe fn cxx_default_new(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::swift_lookup_table::single_entry::small_vector::ffi::cxx_default_new(this)
    }

    #[inline]
    unsafe fn cxx_destruct(this: *mut Self::DefaultType) {
        let this = <Self as SmallVectorElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::swift_lookup_table::single_entry::small_vector::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_ref_small_vector_impl(this: &Self::DefaultType) -> &SmallVectorImpl<Self> {
        let this = <Self as SmallVectorElement>::into_repr_ref(this);
        let this =
            crate::gen::swift::clang_importer::swift_lookup_table::single_entry::small_vector::as_ref_small_vector_impl(
                this,
            );
        <Self as SmallVectorImplElement>::from_repr_ref(this)
    }

    #[inline]
    fn as_pin_small_vector_impl(this: Pin<&mut Self::DefaultType>) -> Pin<&mut SmallVectorImpl<Self>> {
        let this = <Self as SmallVectorElement>::into_repr_pin(this);
        let this =
            crate::gen::swift::clang_importer::swift_lookup_table::single_entry::small_vector::as_pin_small_vector_impl(
                this,
            );
        <Self as SmallVectorImplElement>::from_repr_pin(this)
    }
}

impl<'ctx> SmallVectorImplElement for SwiftLookupTableSingleEntry<'ctx> {
    type ReprType = SwiftLookupTableSingleEntrySmallVectorImpl<'ctx>;

    #[inline]
    unsafe fn cxx_destruct(this: *mut SmallVectorImpl<Self>) {
        let this = <Self as SmallVectorImplElement>::into_repr_mut_ptr(this);
        crate::auto::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::ffi::cxx_destruct(this)
    }

    #[inline]
    fn as_slice(this: &SmallVectorImpl<Self>) -> &[Self] {
        let this = <Self as SmallVectorImplElement>::into_repr_ref(this);
        crate::gen::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::as_slice(this)
    }

    #[inline]
    fn as_pin_slice(this: Pin<&mut SmallVectorImpl<Self>>) -> Pin<&mut [Self]> {
        let this = <Self as SmallVectorImplElement>::into_repr_pin(this);
        let slice = unsafe {
            crate::gen::swift::clang_importer::swift_lookup_table::single_entry::small_vector_impl::as_mut_slice(this)
        };
        unsafe { Pin::new_unchecked(slice) }
    }
}

#[cfg(test)]
mod test {
    use super::SwiftLookupTableSingleEntry;
    use crate::auto::swift::clang_importer::swift_lookup_table::single_entry;
    use cxx_llvm::llvm::adt::{small_vector::SmallVectorElement, small_vector_impl::SmallVectorImpl};

    static_assertions::const_assert_eq!(
        core::mem::size_of::<<SwiftLookupTableSingleEntry as SmallVectorElement>::DefaultType>(),
        core::mem::size_of::<single_entry::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<<SwiftLookupTableSingleEntry as SmallVectorElement>::DefaultType>(),
        core::mem::align_of::<single_entry::small_vector::SmallVector>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::size_of::<SmallVectorImpl<SwiftLookupTableSingleEntry>>(),
        core::mem::size_of::<single_entry::small_vector_impl::SmallVectorImpl>(),
    );

    static_assertions::const_assert_eq!(
        core::mem::align_of::<SmallVectorImpl<SwiftLookupTableSingleEntry>>(),
        core::mem::align_of::<single_entry::small_vector_impl::SmallVectorImpl>(),
    );
}
