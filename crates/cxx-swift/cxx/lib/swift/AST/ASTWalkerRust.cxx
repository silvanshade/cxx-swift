#include "cxx-swift/cxx/include/swift/AST/ASTWalkerRust.hxx"

namespace cxx_swift::swift::ast::ast_walker_rust {

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkIntoGenericParams() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkIntoGenericParams" << std::endl;
  return this->ast_walker_dyn->shouldWalkIntoGenericParams();
}

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkIntoTapExpression() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkIntoTapExpression" << std::endl;
  return this->ast_walker_dyn->shouldWalkIntoTapExpression();
}

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkIntoPropertyWrapperPlaceholderValue() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkIntoPropertyWrapperPlaceholderValue"
  // << std::endl;
  return this->ast_walker_dyn->shouldWalkIntoPropertyWrapperPlaceholderValue();
}

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkCaptureInitializerExpressions() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkCaptureInitializerExpressions" <<
  // std::endl;
  return this->ast_walker_dyn->shouldWalkCaptureInitializerExpressions();
}

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkAccessorsTheOldWay() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkAccessorsTheOldWay" << std::endl;
  return this->ast_walker_dyn->shouldWalkAccessorsTheOldWay();
}

template<>
[[gnu::always_inline]]
auto
ASTWalkerRust::shouldWalkSerializedTopLevelInternalDecls() noexcept -> bool
{
  // std::cout << "ASTWalkerRust::shouldWalkSerializedTopLevelInternalDecls" <<
  // std::endl;
  return this->ast_walker_dyn->shouldWalkSerializedTopLevelInternalDecls();
}

} // namespace cxx_swift::swift::ast::ast_walker_rust
