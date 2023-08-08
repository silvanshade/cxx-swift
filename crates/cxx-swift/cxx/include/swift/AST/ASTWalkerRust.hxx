#pragma once

#include "cxx-auto/cxx/include/cxx-auto.hxx"
#include "cxx-swift-auto/cxx/include/swift/AST/ASTWalkerRust/Class.hxx"
#include "cxx-swift-auto/src/proxy/ast_walker_dyn.rs.h"
#include "cxx-swift/src/gen/swift/ast/ast_walker_rust.rs.h"
#include "rust/cxx.h"
#include "swift/AST/ASTWalker.h"

#include <type_traits>

namespace cxx_swift::swift::ast::ast_walker_rust {
using ASTWalkerRust = ASTWalkerRustTemplate<ASTWalkerDyn>;
using F = ASTWalkerRustTemplate<ASTWalkerDyn>;

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return cxx_auto::cxx_abi_align<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return cxx_auto::cxx_abi_size<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return cxx_auto::cxx_is_default_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return cxx_auto::cxx_is_copy_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return cxx_auto::cxx_is_move_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return cxx_auto::cxx_is_destructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return cxx_auto::cxx_is_trivially_copyable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return cxx_auto::cxx_is_trivially_movable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return cxx_auto::cxx_is_trivially_destructible<F>();
}

} // namespace cxx_swift::swift::ast::ast_walker_rust

namespace cxx_swift::swift::ast::ast_walker_rust {
[[gnu::always_inline]]
static inline auto
cxx_placement_new(
  F* This [[clang::lifetimebound]],
  rust::Box<ASTWalkerDyn> ast_walker_dyn [[clang::lifetimebound]]
) noexcept -> void
{
  return cxx_auto::cxx_placement_new(This, ::std::forward<rust::Box<ASTWalkerDyn>>(ast_walker_dyn));
}

[[gnu::always_inline]]
static inline auto
cxx_move_new(F* This [[clang::lifetimebound]], F* that [[clang::lifetimebound]]) noexcept -> void
{
  F&& that_rvalue = ::std::move(*that);
  return cxx_auto::cxx_move_new(This, ::std::forward<F&&>(that_rvalue));
}

[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_auto::cxx_destruct(This);
}

} // namespace cxx_swift::swift::ast::ast_walker_rust

namespace cxx_swift::swift::ast::ast_walker_rust {
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

} // namespace cxx_swift::swift::ast::ast_walker_rust

namespace cxx_swift::swift::ast::ast_walker_rust {
static_assert(
  cxx_auto::cxx_abi_align<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_abi_align<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_abi_size<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_abi_size<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_copy_constructible<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_copy_constructible<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_move_constructible<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_move_constructible<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_destructible<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_destructible<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_trivially_copyable<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_trivially_copyable<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_trivially_movable<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_trivially_movable<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
static_assert(
  cxx_auto::cxx_is_trivially_destructible<ASTWalkerRustTemplate<ASTWalkerDyn>>() ==
  cxx_auto::cxx_is_trivially_destructible<ASTWalkerRustTemplate<ASTWalkerDynProxy>>()
);
} // namespace cxx_swift::swift::ast::ast_walker_rust
