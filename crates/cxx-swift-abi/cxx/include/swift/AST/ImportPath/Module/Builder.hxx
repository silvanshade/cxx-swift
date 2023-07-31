#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "swift/AST/Identifier.h"
#include "swift/AST/Import.h"

namespace cxx_swift::swift::ast::import_path::module::builder {
CXX_MEMORY_ABI_PRELUDE(Builder, ::swift::ImportPath::Module::Builder)
} // namespace cxx_swift::swift::ast::import_path::module::builder

namespace cxx_swift::swift::ast::import_path::module::builder {
[[gnu::always_inline]] [[gnu::const]]
static inline auto
cxx_placement_new_from_identifier(Self* This [[clang::lifetimebound]], ::swift::Identifier name) noexcept -> void
{
  return cxx_memory::abi::cxx_placement_new(This, name);
}

[[gnu::always_inline]] [[gnu::const]]
static inline auto
get(Self const& This [[clang::lifetimebound]]) noexcept -> ::swift::ImportPath::Module
{
  return This.get();
}

} // namespace cxx_swift::swift::ast::import_path::module::builder
