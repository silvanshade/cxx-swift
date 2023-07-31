#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVector.hxx"
#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "rust/cxx.h"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector {
CXX_MEMORY_ABI_PRELUDE(SmallVector, ::llvm::SmallVector, ::swift::SerializedSwiftName)
} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector {
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

} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector
