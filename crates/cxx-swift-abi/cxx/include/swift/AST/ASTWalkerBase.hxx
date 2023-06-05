#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/AST/ASTWalker.h"

namespace cxx_swift::swift::ast::ast_walker_base {
class ASTWalkerBase : public ::swift::ASTWalker
{

public:
  ASTWalkerBase() noexcept = default;
  ASTWalkerBase(ASTWalkerBase const&) noexcept = delete;
  ASTWalkerBase(ASTWalkerBase&&) noexcept = default;

  ~ASTWalkerBase() override = default;

  auto operator=(const ASTWalkerBase&) noexcept -> ASTWalkerBase& = delete;
  auto operator=(ASTWalkerBase&&) noexcept -> ASTWalkerBase& = default;

  auto shouldWalkIntoGenericParams() noexcept -> bool override;
  auto shouldWalkIntoTapExpression() noexcept -> bool override;
  auto shouldWalkIntoPropertyWrapperPlaceholderValue() noexcept -> bool override;
  auto shouldWalkCaptureInitializerExpressions() noexcept -> bool override;
  auto shouldWalkAccessorsTheOldWay() noexcept -> bool override;
  auto shouldWalkSerializedTopLevelInternalDecls() noexcept -> bool override;
};

} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
using F = ASTWalkerBase;

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

} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
[[gnu::always_inline]]
static inline auto
cxx_default_new(F* This [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_default_new(This);
}

[[gnu::always_inline]]
static inline auto
cxx_move_new(F* This [[clang::lifetimebound]], F* that [[clang::lifetimebound]]) noexcept -> void
{
  F&& that_rvalue = ::std::move(*that);
  return cxx_memory::abi::cxx_move_new(This, ::std::forward<F&&>(that_rvalue));
}

[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
[[gnu::always_inline]]
static inline auto
deref(F const& This [[clang::lifetimebound]]) -> ::swift::ASTWalker const&
{
  return This;
}

[[gnu::always_inline]]
static inline auto
deref_pin(F& This [[clang::lifetimebound]]) -> ::swift::ASTWalker&
{
  return This;
}

} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_generic_params(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoGenericParams();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_tap_expression(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoTapExpression();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_property_wrapper_placeholder_value(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoPropertyWrapperPlaceholderValue();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_capture_initializer_expressions(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkCaptureInitializerExpressions();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_accessors_the_old_way(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkAccessorsTheOldWay();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_serialized_top_level_internal_decls(F& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker_base
