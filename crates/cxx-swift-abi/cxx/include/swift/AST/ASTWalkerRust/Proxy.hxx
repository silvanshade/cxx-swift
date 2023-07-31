#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalkerRust/Class.hxx"
#include "cxx-swift-abi/src/proxy/ast_walker_dyn.rs.h"
#include "rust/cxx.h"
#include "swift/AST/ASTWalker.h"

namespace cxx_swift::swift::ast::ast_walker_rust::proxy {
// NOTE: We define these abi helper functions in terms of `ASTWalkerDynProxy` because `ASTWalkerDyn`
// is a wrapper for a trait object that depends on the definition of `ASTWalker`. That means that in
// order to perfectly compute the alignment and size information for `ASTWalkerRust`, we must
// already have computed that information for `ASTWalker`.
//
// However, cxx does not readily support phased compilation in a fashion where we might be able to
// express this dependency.
//
// Our only options for handling this situation are either to break the definitions into multiple
// crates, so that `ASTWalker` definitions are generated prior to `ASTWalkerDyn` definitions, or use
// a proxy trait object representing `ASTWalkerDyn` and then make the `ASTWalkerRust` class generic
// over the trait object parameter (which works because all trait objects will have the same size
// and alignment regardless of the specific definition of the trait).
//
// We choose the latter approach here, and add some additional static assertions (ensuring all the
// information corresponds correctly) in `../ASTWalker.hxx`.
CXX_MEMORY_ABI_PRELUDE(ASTWalkerRustProxy, ASTWalkerRustTemplate, ASTWalkerDynProxy)
} // namespace cxx_swift::swift::ast::ast_walker_rust::proxy
