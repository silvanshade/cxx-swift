#pragma once

#include "cxx-swift-auto/cxx/include/cxx-swift-auto.hxx"
#include "rust/cxx.h"
#include "swift/AST/ASTContext.h"
#include "swift/AST/DiagnosticEngine.h"

namespace cxx_swift::swift::ast::ast_context {
CXX_AUTO_PRELUDE(ASTContext, ::swift::ASTContext)
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
