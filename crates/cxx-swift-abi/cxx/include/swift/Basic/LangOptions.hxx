#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/Basic/LangOptions.h"

namespace cxx_swift::swift::basic::lang_options {
CXX_MEMORY_ABI_PRELUDE(LangOptions, ::swift::LangOptions)
} // namespace cxx_swift::swift::basic::lang_options

namespace cxx_swift::swift::basic::lang_options {
[[gnu::always_inline]]
static inline auto
set_target(Self& This, ::llvm::Triple* triple, bool& os_was_invalid, bool& arch_was_invalid) noexcept -> void
{
  auto&& value = ::std::move(*triple);
  auto&& pair = This.setTarget(value);
  os_was_invalid = pair.first;
  arch_was_invalid = pair.second;
}

} // namespace cxx_swift::swift::basic::lang_options
