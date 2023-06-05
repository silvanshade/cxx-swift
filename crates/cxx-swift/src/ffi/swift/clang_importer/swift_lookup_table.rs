pub(crate) mod single_entry;

use crate::{
    ffi::swift::clang_importer::{
        effective_clang_context::EffectiveClangContext,
        serialized_swift_name::{owning_iterator::SerializedSwiftNameOwningIterator, SerializedSwiftName},
        swift_lookup_table::single_entry::owning_iterator::SwiftLookupTableSingleEntryOwningIterator,
    },
    gen::swift::clang_importer::swift_lookup_table,
};
use core::pin::Pin;

pub use crate::abi::swift::clang_importer::swift_lookup_table::SwiftLookupTable;

impl<'ctx> SwiftLookupTable<'ctx> {
    #[inline]
    pub fn lookup(
        &self,
        base_name: SerializedSwiftName<'ctx>,
        search_context: EffectiveClangContext,
    ) -> impl cxx_memory::New<Output = SwiftLookupTableSingleEntryOwningIterator<'ctx>> + '_ {
        unsafe {
            cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                swift_lookup_table::lookup(self, base_name, search_context, this)
            })
        }
    }

    #[inline]
    pub fn all_base_names(&self) -> impl cxx_memory::New<Output = SerializedSwiftNameOwningIterator> + '_ {
        unsafe {
            cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                swift_lookup_table::all_base_names(self, this)
            })
        }
    }

    #[inline]
    pub fn deserialize_all(self: Pin<&mut Self>) {
        swift_lookup_table::deserialize_all(self)
    }

    #[inline]
    pub fn dump(self: Pin<&mut Self>) {
        swift_lookup_table::dump(self)
    }
}
