#pragma once

#include "cxx-llvm-auto/cxx/include/llvm/ADT/SmallVector.hxx"
#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

namespace cxx_swift::swift::ast::identifier::small_vector {
CXX_AUTO_PRELUDE(SmallVector, ::llvm::SmallVector, ::swift::Identifier)
} // namespace cxx_swift::swift::ast::identifier::small_vector

namespace cxx_swift::swift::ast::identifier::small_vector {
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

} // namespace cxx_swift::swift::ast::identifier::small_vector
