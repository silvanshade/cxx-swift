pub(crate) mod owning_iterator;

use crate::gen::swift::clang_importer::serialized_swift_name;
use cxx_llvm::llvm::adt::string_ref::StringRef;

pub use crate::abi::swift::clang_importer::serialized_swift_name::SerializedSwiftName;

impl<'ctx> SerializedSwiftName<'ctx> {
    #[inline]
    pub fn get_name(&self) -> StringRef<'ctx> {
        serialized_swift_name::get_name(self)
    }
}
