#pragma once

#include "cxx-llvm-abi/cxx/include/llvm/ADT/SmallVectorBoxed.hxx"
#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed {
using T = ::swift::SwiftLookupTable::SingleEntry;
using SmallVectorBoxed = cxx_llvm::llvm::adt::small_vector_boxed::SmallVectorBoxed<T>;
using F = SmallVectorBoxed;

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

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed {
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

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_ref_small_vector_impl(F const& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<T> const&
{
  return This.as_ref_small_vector_impl();
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_pin_small_vector_impl(F& This [[clang::lifetimebound]]) noexcept -> ::llvm::SmallVectorImpl<T>&
{
  return This.as_pin_small_vector_impl();
}

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry::small_vector_boxed
