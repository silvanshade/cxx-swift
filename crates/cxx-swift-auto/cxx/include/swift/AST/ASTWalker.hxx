#pragma once

#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "swift/AST/ASTWalker.h"
#include "swift/AST/Expr.h"

namespace cxx_swift::swift::ast::ast_walker {
CXX_AUTO_PRELUDE(ASTWalker, ::swift::ASTWalker)
} // namespace cxx_swift::swift::ast::ast_walker

namespace cxx_swift::swift::ast::ast_walker {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_parent(Self const& This [[clang::lifetimebound]]) noexcept -> Self::ParentTy
{
  return This.Parent;
}

[[gnu::always_inline]]
static inline auto
walk_to_expr_pre(
  Self& This [[clang::lifetimebound]],
  ::swift::Expr* E,
  Self::PreWalkResult<::swift::Expr*>* out
) noexcept -> void
{
  auto&& result = This.walkToExprPre(E);
  // NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg)
  new (out) Self::PreWalkResult<::swift::Expr*>{ std::move(result) };
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_generic_params(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoGenericParams();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_tap_expression(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoTapExpression();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_property_wrapper_placeholder_value(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoPropertyWrapperPlaceholderValue();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_capture_initializer_expressions(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkCaptureInitializerExpressions();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_accessors_the_old_way(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkAccessorsTheOldWay();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_serialized_top_level_internal_decls(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker
