#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorBoxed.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::swift_lookup_table {
CXX_MEMORY_ABI_PRELUDE(SwiftLookupTable, ::swift::SwiftLookupTable)
} // namespace cxx_swift::swift::clang_importer::swift_lookup_table

namespace cxx_swift::swift::clang_importer::swift_lookup_table {
template<typename T>
using SmallVectorBoxed = ::cxx_llvm::llvm::adt::small_vector_boxed::SmallVectorBoxed<T>;

[[gnu::always_inline]]
static inline auto
lookup(
  Self const& This [[clang::lifetimebound]],
  ::swift::SerializedSwiftName base_name,
  ::swift::EffectiveClangContext search_context,
  SmallVectorBoxed<::swift::SwiftLookupTable::SingleEntry>* out
) noexcept -> void
{
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-const-cast)
  auto&& container = const_cast<Self&>(This).lookup(base_name, search_context);
  new (out) SmallVectorBoxed<::swift::SwiftLookupTable::SingleEntry>{ std::move(container) };
}

[[gnu::always_inline]]
static inline auto
all_base_names(Self const& This [[clang::lifetimebound]], SmallVectorBoxed<::swift::SerializedSwiftName>* out) noexcept
  -> void
{
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-const-cast)
  auto&& container = const_cast<Self&>(This).allBaseNames();
  new (out) SmallVectorBoxed<::swift::SerializedSwiftName>{ std::move(container) };
}

[[gnu::always_inline]]
static inline auto
deserialize_all(Self& This [[clang::lifetimebound]]) noexcept -> void
{
  return This.deserializeAll();
}

[[gnu::always_inline]]
static inline auto
dump(Self& This [[clang::lifetimebound]]) noexcept -> void
{
  return This.dump(::llvm::outs());
}

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table
