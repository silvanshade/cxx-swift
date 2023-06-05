#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/AST/ASTWalker.h"
#include "swift/AST/Expr.h"

namespace cxx_swift::swift::ast::ast_walker {
using ASTWalker = ::swift::ASTWalker;
using F = ASTWalker;

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_align<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_size<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_default_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_copy_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_move_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_destructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_copyable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_movable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_destructible<F>();
}

} // namespace cxx_swift::swift::ast::ast_walker

namespace cxx_swift::swift::ast::ast_walker {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
get_parent(F const& This [[clang::lifetimebound]]) noexcept -> F::ParentTy
{
  return This.Parent;
}

[[gnu::always_inline]]
static inline auto
walk_to_expr_pre(F& This [[clang::lifetimebound]], ::swift::Expr* E, F::PreWalkResult<::swift::Expr*>* out) noexcept
  -> void
{
  *out = This.walkToExprPre(E);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_generic_params(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoGenericParams();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_tap_expression(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoTapExpression();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_into_property_wrapper_placeholder_value(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoPropertyWrapperPlaceholderValue();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_capture_initializer_expressions(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkCaptureInitializerExpressions();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_accessors_the_old_way(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkAccessorsTheOldWay();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
should_walk_serialized_top_level_internal_decls(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker
