#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorBoxed.hxx"
#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_boxed {
CXX_MEMORY_ABI_PRELUDE(
  SmallVectorBoxed,
  cxx_llvm::llvm::adt::small_vector_boxed::SmallVectorBoxed,
  ::swift::SerializedSwiftName
)
} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_boxed

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_boxed {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_small_vector_impl(Self const& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<TyArg0> const&
{
  return This.as_ref_small_vector_impl();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_small_vector_impl(Self& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<TyArg0>&
{
  return This.as_pin_small_vector_impl();
}

} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_boxed
