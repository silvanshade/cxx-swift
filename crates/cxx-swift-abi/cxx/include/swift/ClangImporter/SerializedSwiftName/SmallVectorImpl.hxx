#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorImpl.hxx"
#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "rust/cxx.h"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl {
CXX_MEMORY_ABI_PRELUDE(SmallVectorImpl, ::llvm::SmallVectorImpl, ::swift::SerializedSwiftName)
} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl

namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl {
[[gnu::always_inline]]
static inline auto
as_slice(Self const& This [[clang::lifetimebound]]) -> rust::Slice<const TyArg0>
{
  return rust::Slice{ This.data(), This.size() };
}

[[gnu::always_inline]]
static inline auto
as_mut_slice(Self& This [[clang::lifetimebound]]) -> rust::Slice<TyArg0>
{
  return rust::Slice{ This.data(), This.size() };
}

} // namespace cxx_swift::swift::clang_importer::serialized_swift_name::small_vector_impl
