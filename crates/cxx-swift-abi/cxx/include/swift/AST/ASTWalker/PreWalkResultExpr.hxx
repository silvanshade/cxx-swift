#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "swift/AST/ASTWalker.h"
#include "swift/AST/Expr.h"

namespace cxx_memory::abi {
template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_cxx_memory_copy_new<::swift::ASTWalker::PreWalkResult<::swift::Expr*>>() noexcept -> bool
{
  return false;
}

template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_cxx_memory_move_new<::swift::ASTWalker::PreWalkResult<::swift::Expr*>>() noexcept -> bool
{
  return false;
}

} // namespace cxx_memory::abi

namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr {
CXX_MEMORY_ABI_PRELUDE(PreWalkResultExpr, ::swift::ASTWalker::PreWalkResult, ::swift::Expr*)
} // namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr
