#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/Basic/LangOptions.h"

namespace cxx_swift::swift::basic::clang_importer_options {
using ClangImporterOptions = ::swift::ClangImporterOptions;
using F = ClangImporterOptions;

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

} // namespace cxx_swift::swift::basic::clang_importer_options

namespace cxx_swift::swift::basic::clang_importer_options {
[[gnu::always_inline]] [[gnu::pure]]
static inline auto
modify_extra_args_push_back(F& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.ExtraArgs.emplace_back(value);
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_bridging_header(F& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.BridgingHeader = value;
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_module_cache_path(F& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.ModuleCachePath = value;
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_precompiled_header_output_dir(F& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept
  -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.PrecompiledHeaderOutputDir = value;
}

} // namespace cxx_swift::swift::basic::clang_importer_options

namespace cxx_swift::swift::basic::clang_importer_options {
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

} // namespace cxx_swift::swift::basic::clang_importer_options
