#pragma once

#include "swift/Basic/LLVMInitialize.h"

namespace cxx_swift::swift::basic::llvm_initialize {
[[gnu::always_inline]]
static inline auto
initialize_llvm() noexcept -> void
{
  // NOLINTNEXTLINE(cppcoreguidelines-avoid-do-while)
  INITIALIZE_LLVM();
}

} // namespace cxx_swift::swift::basic::llvm_initialize
