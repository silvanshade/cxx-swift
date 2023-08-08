#pragma once

#include "cxx-llvm-auto/cxx/include/llvm/ADT/SmallVectorImpl.hxx"
#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

namespace cxx_swift::swift::ast::identifier::small_vector_impl {
CXX_AUTO_PRELUDE(SmallVectorImpl, ::llvm::SmallVectorImpl, ::swift::Identifier)
} // namespace cxx_swift::swift::ast::identifier::small_vector_impl

namespace cxx_swift::swift::ast::identifier::small_vector_impl {
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

} // namespace cxx_swift::swift::ast::identifier::small_vector_impl
