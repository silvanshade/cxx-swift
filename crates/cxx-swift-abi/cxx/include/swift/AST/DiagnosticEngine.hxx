#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "swift/AST/DiagnosticEngine.h"

namespace cxx_swift::swift::ast::diagnostic_engine {
CXX_MEMORY_ABI_PRELUDE(DiagnosticEngine, ::swift::DiagnosticEngine)
} // namespace cxx_swift::swift::ast::diagnostic_engine

namespace cxx_swift::swift::ast::diagnostic_engine {
[[gnu::always_inline]] [[gnu::const]]
static inline auto
cxx_placement_new_from_source_manager(
  Self* This [[clang::lifetimebound]],
  ::swift::SourceManager& source_manager [[clang::lifetimebound]]
) noexcept -> void
{
  return cxx_memory::abi::cxx_placement_new(This, std::forward<::swift::SourceManager&>(source_manager));
}

} // namespace cxx_swift::swift::ast::diagnostic_engine
