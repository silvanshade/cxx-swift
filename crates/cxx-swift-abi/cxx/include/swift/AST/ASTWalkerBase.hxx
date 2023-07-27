#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/AST/ASTWalker.h"

namespace cxx_swift::swift::ast::ast_walker_base::detail {
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

} // namespace cxx_swift::swift::ast::ast_walker_base::detail

namespace cxx_swift::swift::ast::ast_walker_base {
CXX_MEMORY_ABI_PRELUDE(ASTWalkerBase, detail::ASTWalkerBase)
} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
[[gnu::always_inline]]
static inline auto
deref(Self const& This [[clang::lifetimebound]]) -> ::swift::ASTWalker const&
{
  return This;
}

[[gnu::always_inline]]
static inline auto
deref_pin(Self& This [[clang::lifetimebound]]) -> ::swift::ASTWalker&
{
  return This;
}

} // namespace cxx_swift::swift::ast::ast_walker_base

namespace cxx_swift::swift::ast::ast_walker_base {
[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_generic_params(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoGenericParams();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_tap_expression(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoTapExpression();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_into_property_wrapper_placeholder_value(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkIntoPropertyWrapperPlaceholderValue();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_capture_initializer_expressions(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkCaptureInitializerExpressions();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_accessors_the_old_way(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkAccessorsTheOldWay();
}

[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
should_walk_serialized_top_level_internal_decls(Self& This [[clang::lifetimebound]]) noexcept -> bool
{
  return This.shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker_base
