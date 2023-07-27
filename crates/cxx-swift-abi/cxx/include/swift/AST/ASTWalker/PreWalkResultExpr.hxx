#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/AST/ASTWalker.h"
#include "swift/AST/Expr.h"

namespace cxx_memory::abi {
using T = ::swift::ASTWalker::PreWalkResult<::swift::Expr*>;

template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_cxx_memory_copy_new<T>() noexcept -> bool
{
  return false;
}

template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_cxx_memory_move_new<T>() noexcept -> bool
{
  return false;
}

} // namespace cxx_memory::abi

namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr {
CXX_MEMORY_ABI_PRELUDE(PreWalkResultExpr, ::swift::ASTWalker::PreWalkResult, ::swift::Expr*)
} // namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr
