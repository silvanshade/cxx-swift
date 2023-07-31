#include "cxx-swift-abi/cxx/include/cxx-swift-abi.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTContext.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalker.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalker/ParentTy.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalker/PreWalkResultExpr.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalkerBase.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalkerRust/Class.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ASTWalkerRust/Proxy.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Decl.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/DependencyTracker.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/DiagnosticEngine.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Expr.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Identifier.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Identifier/SmallVector.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Identifier/SmallVectorImpl.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ImportPath/Module.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ImportPath/Module/Builder.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/ModuleDecl.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Pattern.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/SILOptions.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/SearchPathOptions.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/Stmt.hxx"
#include "cxx-swift-abi/cxx/include/swift/AST/TypeRepr.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/ClangImporterOptions.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/LLVMInitialize.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/LangOptions.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/SourceLoc.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/SourceManager.hxx"
#include "cxx-swift-abi/cxx/include/swift/Basic/TypeCheckerOptions.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/ClangImporter.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/EffectiveClangContext.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVector.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVectorBoxed.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName/SmallVectorImpl.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVector.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorBoxed.hxx"
#include "cxx-swift-abi/cxx/include/swift/ClangImporter/SwiftLookupTable/SingleEntry/SmallVectorImpl.hxx"
#include "cxx-swift-abi/cxx/include/swift/SymbolGraphGen/SymbolGraphOptions.hxx"
