#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/ASTContext.h"
#include "swift/AST/DiagnosticEngine.h"

namespace cxx_swift::swift::ast::ast_context {
using ASTContext = ::swift::ASTContext;
using F = ASTContext;

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

} // namespace cxx_swift::swift::ast::ast_context

namespace cxx_swift::swift::ast::ast_context {
[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_swift::swift::ast::ast_context

namespace cxx_swift::swift::ast::ast_context {
[[gnu::always_inline]]
static inline auto
get(
  ::swift::LangOptions& lang_options [[clang::lifetimebound]],
  ::swift::TypeCheckerOptions& type_checker_options [[clang::lifetimebound]],
  ::swift::SILOptions& sil_options [[clang::lifetimebound]],
  ::swift::SearchPathOptions& search_path_options [[clang::lifetimebound]],
  ::swift::ClangImporterOptions& clang_importer_options [[clang::lifetimebound]],
  ::swift::symbolgraphgen::SymbolGraphOptions& symbol_graph_options [[clang::lifetimebound]],
  ::swift::DiagnosticEngine& diagnostic_engine [[clang::lifetimebound]],
  rust::Fn<bool(::llvm::StringRef, bool)> callback
) -> ::std::unique_ptr<::swift::ASTContext>
{
  auto* ptr = ::swift::ASTContext::get(
    lang_options,
    type_checker_options,
    sil_options,
    search_path_options,
    clang_importer_options,
    symbol_graph_options,
    diagnostic_engine.SourceMgr,
    diagnostic_engine,
    callback
  );
  return ::std::unique_ptr<::swift::ASTContext>(ptr);
}

} // namespace cxx_swift::swift::ast::ast_context
