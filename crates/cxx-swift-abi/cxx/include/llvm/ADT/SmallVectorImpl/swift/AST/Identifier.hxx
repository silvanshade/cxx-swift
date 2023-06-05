#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorImpl.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier {
using T = ::swift::Identifier;
using SmallVectorImpl = ::llvm::SmallVectorImpl<T>;

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

} // namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier

namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier {
[[gnu::always_inline]]
static inline auto
cxx_destruct(F<T>* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier

namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier {
[[gnu::always_inline]]
static inline auto
as_slice(F<T> const& This [[clang::lifetimebound]]) -> rust::Slice<const T>
{
  return rust::Slice{ This.data(), This.size() };
}

[[gnu::always_inline]]
static inline auto
as_mut_slice(F<T>& This [[clang::lifetimebound]]) -> rust::Slice<T>
{
  return rust::Slice{ This.data(), This.size() };
}

} // namespace cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier
