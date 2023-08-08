#pragma once

#include "cxx-llvm-auto/cxx/include/llvm/ADT/SmallVector.hxx"
#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "rust/cxx.h"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector {
CXX_AUTO_PRELUDE(SmallVector, ::llvm::SmallVector, ::swift::SwiftLookupTable::SingleEntry)
} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector {
[[gnu::always_inline]]
static inline auto
as_ref_small_vector_impl(Self const& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<TyArg0> const&
{
  return This;
}

[[gnu::always_inline]]
static inline auto
as_pin_small_vector_impl(Self& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<TyArg0>&
{
  return This;
}

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector
