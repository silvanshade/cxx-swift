#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "rust/cxx.h"
#include "swift/Basic/LangOptions.h"

namespace cxx_swift::swift::basic::clang_importer_options {
CXX_MEMORY_ABI_PRELUDE(ClangImporterOptions, ::swift::ClangImporterOptions)
} // namespace cxx_swift::swift::basic::clang_importer_options

namespace cxx_swift::swift::basic::clang_importer_options {
[[gnu::always_inline]] [[gnu::pure]]
static inline auto
modify_extra_args_push_back(Self& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.ExtraArgs.emplace_back(value);
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_bridging_header(Self& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.BridgingHeader = value;
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_module_cache_path(Self& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.ModuleCachePath = value;
}

[[gnu::always_inline]] [[gnu::pure]]
static inline auto
set_precompiled_header_output_dir(Self& This [[clang::lifetimebound]], rust::Slice<char const> const slice) noexcept
  -> void
{
  auto&& value = ::std::string(slice.data(), slice.size());
  This.PrecompiledHeaderOutputDir = value;
}

} // namespace cxx_swift::swift::basic::clang_importer_options
