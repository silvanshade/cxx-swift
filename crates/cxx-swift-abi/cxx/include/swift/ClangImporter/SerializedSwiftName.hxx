#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::serialized_swift_name {
CXX_MEMORY_ABI_PRELUDE(SerializedSwiftName, ::swift::SerializedSwiftName)
} // namespace cxx_swift::swift::clang_importer::serialized_swift_name

namespace cxx_swift::swift::clang_importer::serialized_swift_name {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_name(Self const& This [[clang::lifetimebound]]) noexcept -> ::llvm::StringRef
{
  return This.Name;
}

} // namespace cxx_swift::swift::clang_importer::serialized_swift_name
