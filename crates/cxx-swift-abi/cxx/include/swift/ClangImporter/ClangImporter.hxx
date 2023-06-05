#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/AST/ClangModuleLoader.h"
#include "swift/AST/Import.h"
#include "swift/Basic/SourceLoc.h"
#include "swift/ClangImporter/ClangImporter.h"

namespace cxx_swift::swift::clang_importer::clang_importer {
using ClangImporter = ::swift::ClangImporter;
using F = ClangImporter;

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

} // namespace cxx_swift::swift::clang_importer::clang_importer

namespace cxx_swift::swift::clang_importer::clang_importer {
[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_swift::swift::clang_importer::clang_importer

namespace cxx_swift::swift::clang_importer::clang_importer {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
create(
  ::swift::ASTContext& ast_context [[clang::lifetimebound]],
  ::std::string* swift_pch_hash_p [[clang::lifetimebound]],
  ::swift::DependencyTracker* tracker [[clang::lifetimebound]]
) noexcept -> ::std::unique_ptr<F>
{
  ::std::string swift_pch_hash = swift_pch_hash_p == nullptr ? "" : *swift_pch_hash_p;
  return F::create(ast_context, swift_pch_hash, tracker);
}

[[gnu::always_inline]]
static inline auto
collect_visible_top_level_module_names(
  F& This [[clang::lifetimebound]],
  ::llvm::SmallVectorImpl<::swift::Identifier>& module_names [[clang::lifetimebound]]
) noexcept -> void
{
  return This.collectVisibleTopLevelModuleNames(module_names);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
load_module(
  F& This [[clang::lifetimebound]],
  ::swift::SourceLoc import_loc,
  ::swift::ImportPath::Module module_path,
  bool allow_memory_cache
) noexcept -> ::swift::ModuleDecl const*
{
  return This.loadModule(import_loc, module_path, allow_memory_cache);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
emit_bridging_pch(
  F& This [[clang::lifetimebound]],
  ::llvm::StringRef header_path,
  ::llvm::StringRef output_pch_path
) noexcept -> bool
{
  return This.emitBridgingPCH(header_path, output_pch_path);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
can_read_pch(F const& This [[clang::lifetimebound]], ::llvm::StringRef pch_path) noexcept -> bool
{
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-const-cast)
  return const_cast<F&>(This).canReadPCH(pch_path);
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
find_lookup_table_for_module(F const& This [[clang::lifetimebound]], ::clang::Module const& clang_module) noexcept
  -> ::swift::SwiftLookupTable*
{
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-const-cast)
  return const_cast<F&>(This).findLookupTable(&clang_module);
}

} // namespace cxx_swift::swift::clang_importer::clang_importer
