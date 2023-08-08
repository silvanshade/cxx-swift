#include "cxx-swift-auto/cxx/include/swift/AST/ASTWalkerBase.hxx"

namespace cxx_swift::swift::ast::ast_walker_base {

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkIntoGenericParams() noexcept -> bool
{
  return this->ASTWalker::shouldWalkIntoGenericParams();
}

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkIntoTapExpression() noexcept -> bool
{
  return this->ASTWalker::shouldWalkIntoTapExpression();
}

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkIntoPropertyWrapperPlaceholderValue() noexcept -> bool
{
  return this->ASTWalker::shouldWalkIntoPropertyWrapperPlaceholderValue();
}

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkCaptureInitializerExpressions() noexcept -> bool
{
  return this->ASTWalker::shouldWalkCaptureInitializerExpressions();
}

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkAccessorsTheOldWay() noexcept -> bool
{
  return this->ASTWalker::shouldWalkAccessorsTheOldWay();
}

[[gnu::always_inline]]
auto
ASTWalkerBase::shouldWalkSerializedTopLevelInternalDecls() noexcept -> bool
{
  return this->ASTWalker::shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker_base
