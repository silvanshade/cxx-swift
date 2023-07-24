#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVector.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

namespace cxx_swift::swift::ast::identifier::small_vector {
using T = ::swift::Identifier;
using SmallVector = ::llvm::SmallVector<T>;
using F = SmallVector;

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

} // namespace cxx_swift::swift::ast::identifier::small_vector

namespace cxx_swift::swift::ast::identifier::small_vector {
[[gnu::always_inline]]
static inline auto
cxx_default_new(F* This [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_default_new(This);
}

[[gnu::always_inline]]
static inline auto
cxx_copy_new(F* This [[clang::lifetimebound]], F const& that [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_copy_new(This, that);
}

[[gnu::always_inline]]
static inline auto
cxx_move_new(F* This [[clang::lifetimebound]], F* that [[clang::lifetimebound]]) noexcept -> void
{
  // NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg)
  F&& that_rvalue = ::std::move(*that);
  return cxx_memory::abi::cxx_move_new(This, ::std::forward<F&&>(that_rvalue));
}

[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_swift::swift::ast::identifier::small_vector

namespace cxx_swift::swift::ast::identifier::small_vector {
[[gnu::always_inline]]
static inline auto
as_ref_small_vector_impl(F const& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<T> const&
{
  return This;
}

[[gnu::always_inline]]
static inline auto
as_pin_small_vector_impl(F& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<T>&
{
  return This;
}

} // namespace cxx_swift::swift::ast::identifier::small_vector
