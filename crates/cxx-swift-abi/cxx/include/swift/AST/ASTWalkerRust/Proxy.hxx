#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalkerRust/Class.hxx"
#include "cxx-swift-abi/src/proxy/ast_walker_dyn.rs.h"
#include "rust/cxx.h"
#include "swift/AST/ASTWalker.h"

namespace cxx_swift::swift::ast::ast_walker_rust::proxy {
// NOTE: We define these abi helper functions in terms of `ASTWalkerDynProxy` because `ASTWalkerDyn`
// is a wrapper for a trait object that depends on the definition of `ASTWalker`. That means that in
// order to perfectly compute the alignment and size information for `ASTWalkerRust`, we must
// already have computed that information for `ASTWalker`.
//
// However, cxx does not readily support phased compilation in a fashion where we might be able to
// express this dependency.
//
// Our only options for handling this situation are either to break the definitions into multiple
// crates, so that `ASTWalker` definitions are generated prior to `ASTWalkerDyn` definitions, or use
// a proxy trait object representing `ASTWalkerDyn` and then make the `ASTWalkerRust` class generic
// over the trait object parameter (which works because all trait objects will have the same size
// and alignment regardless of the specific definition of the trait).
//
// We choose the latter approach here, and add some additional static assertions (ensuring all the
// information corresponds correctly) in `../ASTWalker.hxx`.
using F = ASTWalkerRustTemplate<ASTWalkerDynProxy>;

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

} // namespace cxx_swift::swift::ast::ast_walker_rust::proxy
