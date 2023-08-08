#pragma once

#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "swift/AST/Module.h"

#include "clang/Basic/Module.h"

namespace cxx_auto {
template<>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr inline auto
cxx_is_destructible<::swift::ModuleDecl>() noexcept -> bool
{
  return false;
}

} // namespace cxx_auto

namespace cxx_swift::swift::ast::module_decl {
CXX_AUTO_PRELUDE(ModuleDecl, ::swift::ModuleDecl)
} // namespace cxx_swift::swift::ast::module_decl

namespace cxx_swift::swift::ast::module_decl {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
find_underlying_clang_module(Self const& This [[clang::lifetimebound]]) noexcept -> ::clang::Module const*
{
  return This.findUnderlyingClangModule();
}

} // namespace cxx_swift::swift::ast::module_decl
