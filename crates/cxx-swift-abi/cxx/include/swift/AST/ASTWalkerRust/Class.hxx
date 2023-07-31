#pragma once

#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "rust/cxx.h"
#include "swift/AST/ASTWalker.h"

namespace cxx_swift::swift::ast::ast_walker_rust {
template<typename T>
class ASTWalkerRustTemplate : public ::swift::ASTWalker
{
  rust::Box<T> ast_walker_dyn;

public:
  explicit ASTWalkerRustTemplate(rust::Box<T> ast_walker_dyn) noexcept
    : ast_walker_dyn(std::move(ast_walker_dyn))
  {
  }

  ASTWalkerRustTemplate() noexcept = delete;
  ASTWalkerRustTemplate(ASTWalkerRustTemplate const&) noexcept = delete;
  ASTWalkerRustTemplate(ASTWalkerRustTemplate&&) noexcept = default;

  ~ASTWalkerRustTemplate() override = default;

  auto operator=(const ASTWalkerRustTemplate&) noexcept -> ASTWalkerRustTemplate& = delete;
  auto operator=(ASTWalkerRustTemplate&&) noexcept -> ASTWalkerRustTemplate& = default;

  auto shouldWalkIntoGenericParams() noexcept -> bool override;
  auto shouldWalkIntoTapExpression() noexcept -> bool override;
  auto shouldWalkIntoPropertyWrapperPlaceholderValue() noexcept -> bool override;
  auto shouldWalkCaptureInitializerExpressions() noexcept -> bool override;
  auto shouldWalkAccessorsTheOldWay() noexcept -> bool override;
  auto shouldWalkSerializedTopLevelInternalDecls() noexcept -> bool override;
};

} // namespace cxx_swift::swift::ast::ast_walker_rust
