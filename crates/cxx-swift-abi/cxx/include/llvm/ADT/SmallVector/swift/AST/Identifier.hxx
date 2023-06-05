#pragma once

#include "../../../SmallVectorImpl/swift/AST/Identifier.hxx"
#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVector.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier {
using T = ::swift::Identifier;
using SmallVector = ::llvm::SmallVector<T>;

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_align<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_size<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_default_constructible<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_copy_constructible<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_move_constructible<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_destructible<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_copyable<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_movable<F<T>>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_destructible<F<T>>();
}

} // namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier

namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier {
[[gnu::always_inline]]
static inline auto
cxx_default_new(F<T>* This [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_default_new(This);
}

[[gnu::always_inline]]
static inline auto
cxx_copy_new(F<T>* This [[clang::lifetimebound]], F<T> const& that [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_copy_new(This, that);
}

[[gnu::always_inline]]
static inline auto
cxx_move_new(F<T>* This [[clang::lifetimebound]], F<T>* that [[clang::lifetimebound]]) noexcept -> void
{
  // NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg)
  F<T>&& that_rvalue = ::std::move(*that);
  return cxx_memory::abi::cxx_move_new(This, ::std::forward<F<T>&&>(that_rvalue));
}

[[gnu::always_inline]]
static inline auto
cxx_destruct(F<T>* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier

namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier {
[[gnu::always_inline]]
static inline auto
as_ref_small_vector_impl(F<T> const& This [[clang::lifetimebound]]) noexcept
  -> cxx_llvm::llvm::adt::small_vector_impl::F<T> const&
{
  return This;
}

[[gnu::always_inline]]
static inline auto
as_pin_small_vector_impl(F<T>& This [[clang::lifetimebound]]) noexcept -> cxx_llvm::llvm::adt::small_vector_impl::F<T>&
{
  return This;
}

} // namespace cxx_llvm::llvm::adt::small_vector::swift::ast::identifier
