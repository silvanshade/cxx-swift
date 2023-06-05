#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

#include "clang/AST/Decl.h"
#include "clang/Lex/MacroInfo.h"

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry {
using SwiftLookupTableSingleEntry = ::swift::SwiftLookupTable::SingleEntry;
using F = SwiftLookupTableSingleEntry;

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

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry {
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

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_named_decl(F const& This [[clang::lifetimebound]]) noexcept -> ::clang::NamedDecl const*
{
  return This.is<clang::NamedDecl*>() ? This.get<clang::NamedDecl*>() : nullptr;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_macro_info(F const& This [[clang::lifetimebound]]) noexcept -> ::clang::MacroInfo const*
{
  return This.is<::clang::MacroInfo*>() ? This.get<::clang::MacroInfo*>() : nullptr;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_module_macro(F const& This [[clang::lifetimebound]]) noexcept -> ::clang::ModuleMacro const*
{
  return This.is<::clang::ModuleMacro*>() ? This.get<::clang::ModuleMacro*>() : nullptr;
}

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry
