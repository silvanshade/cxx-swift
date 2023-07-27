#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "swift/lib/ClangImporter/SwiftLookupTable.h"

#include "clang/AST/Decl.h"
#include "clang/Lex/MacroInfo.h"

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry {
CXX_MEMORY_ABI_PRELUDE(SwiftLookupTableSingleEntry, ::swift::SwiftLookupTable::SingleEntry)
} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry

namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry {
[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_named_decl(Self const& This [[clang::lifetimebound]]) noexcept -> ::clang::NamedDecl const*
{
  return This.is<clang::NamedDecl*>() ? This.get<clang::NamedDecl*>() : nullptr;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_macro_info(Self const& This [[clang::lifetimebound]]) noexcept -> ::clang::MacroInfo const*
{
  return This.is<::clang::MacroInfo*>() ? This.get<::clang::MacroInfo*>() : nullptr;
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
cast_as_module_macro(Self const& This [[clang::lifetimebound]]) noexcept -> ::clang::ModuleMacro const*
{
  return This.is<::clang::ModuleMacro*>() ? This.get<::clang::ModuleMacro*>() : nullptr;
}

} // namespace cxx_swift::swift::clang_importer::swift_lookup_table::single_entry
