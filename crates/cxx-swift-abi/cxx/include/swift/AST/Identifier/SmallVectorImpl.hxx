#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorImpl.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/Identifier.h"

namespace cxx_swift::swift::ast::identifier::small_vector_impl {
using T = ::swift::Identifier;
using SmallVectorImpl = ::llvm::SmallVectorImpl<T>;
using F = SmallVectorImpl;

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

} // namespace cxx_swift::swift::ast::identifier::small_vector_impl

namespace cxx_swift::swift::ast::identifier::small_vector_impl {
[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_swift::swift::ast::identifier::small_vector_impl

namespace cxx_swift::swift::ast::identifier::small_vector_impl {
[[gnu::always_inline]]
static inline auto
as_slice(F const& This [[clang::lifetimebound]]) -> rust::Slice<const T>
{
  return rust::Slice{ This.data(), This.size() };
}

[[gnu::always_inline]]
static inline auto
as_mut_slice(F& This [[clang::lifetimebound]]) -> rust::Slice<T>
{
  return rust::Slice{ This.data(), This.size() };
}

} // namespace cxx_swift::swift::ast::identifier::small_vector_impl
