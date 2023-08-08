#pragma once

#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "swift/AST/DiagnosticEngine.h"

namespace cxx_swift::swift::ast::diagnostic_engine {
CXX_AUTO_PRELUDE(DiagnosticEngine, ::swift::DiagnosticEngine)
} // namespace cxx_swift::swift::ast::diagnostic_engine

namespace cxx_swift::swift::ast::diagnostic_engine {
[[gnu::always_inline]] [[gnu::const]]
static inline auto
cxx_placement_new_from_source_manager(
  Self* This [[clang::lifetimebound]],
  ::swift::SourceManager& source_manager [[clang::lifetimebound]]
) noexcept -> void
{
  return cxx_auto::cxx_placement_new(This, std::forward<::swift::SourceManager&>(source_manager));
}

} // namespace cxx_swift::swift::ast::diagnostic_engine
