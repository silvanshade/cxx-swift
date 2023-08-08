pub(crate) mod single_entry;

use crate::{
    ffi::swift::clang_importer::{
        effective_clang_context::EffectiveClangContext,
        serialized_swift_name::{
            small_vector_boxed::SmallVectorBoxed as SerializedSwiftNameSmallVectorBoxed,
            SerializedSwiftName,
        },
        swift_lookup_table::single_entry::small_vector_boxed::SmallVectorBoxed as SwiftLookupTableSingleEntrySmallVectorBoxed,
    },
    gen::swift::clang_importer::swift_lookup_table,
};
use core::pin::Pin;

pub use crate::auto::swift::clang_importer::swift_lookup_table::SwiftLookupTable;

impl<'ctx> SwiftLookupTable<'ctx> {
    #[inline]
    pub fn lookup(
        &self,
        base_name: SerializedSwiftName<'ctx>,
        search_context: EffectiveClangContext,
    ) -> impl moveref::New<Output = SwiftLookupTableSingleEntrySmallVectorBoxed<'ctx>> + '_ {
        unsafe {
            moveref::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                swift_lookup_table::lookup(self, base_name, search_context, this)
            })
        }
    }

    #[inline]
    pub fn all_base_names(&self) -> impl moveref::New<Output = SerializedSwiftNameSmallVectorBoxed> + '_ {
        unsafe {
            moveref::new::by_raw(move |this| {
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
