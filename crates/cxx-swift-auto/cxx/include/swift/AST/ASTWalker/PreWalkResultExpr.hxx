#pragma once

#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "swift/AST/ASTWalker.h"
#include "swift/AST/Expr.h"

namespace cxx_auto {
template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_moveref_copy_new<::swift::ASTWalker::PreWalkResult<::swift::Expr*>>() noexcept -> bool
{
  return false;
}

template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
rust_should_impl_moveref_move_new<::swift::ASTWalker::PreWalkResult<::swift::Expr*>>() noexcept -> bool
{
  return false;
}

} // namespace cxx_auto

namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr {
CXX_AUTO_PRELUDE(PreWalkResultExpr, ::swift::ASTWalker::PreWalkResult, ::swift::Expr*)
} // namespace cxx_swift::swift::ast::ast_walker::pre_walk_result_expr
