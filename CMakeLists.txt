# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-swift LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# ensure that the C++ compiler is `clang++`
set(CMAKE_CXX_COMPILER "clang++")

add_library(cxx-swift-abi STATIC
  crates/cxx-swift/cxx/lib/swift/AST/ASTWalkerBase.cxx
  crates/cxx-swift/cxx/lib/swift/AST/ASTWalkerRust.cxx
  crates/cxx-swift-abi/cxx/lib/cmake.cxx
)
target_include_directories(cxx-swift-abi PUBLIC
  ../cxx-clang/crates/cxx-clang-abi/..
  ../cxx-llvm/crates/cxx-llvm-abi/..
  ../cxx-swift/crates/cxx-swift-abi/..
  ../cxx-memory/crates/cxx-memory-abi/..
  crates
  target/cxxbridge
  target/cxxbridge/rust
  ../swift-project/build/Ninja-MinSizeRelAssert/llvm-linux-x86_64/include
  ../swift-project/build/Ninja-MinSizeRelAssert/llvm-linux-x86_64/tools/clang/include
  ../swift-project/build/Ninja-MinSizeRelAssert/swift-linux-x86_64/include
  ../swift-project/llvm-project/clang/include
  ../swift-project/llvm-project/llvm/include
  ../swift-project
  ../swift-project/swift/include
  ../swift-project/swift/stdlib/public/SwiftShims
)
target_compile_definitions(cxx-swift-abi PUBLIC
  _LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS
  _LIBCPP_ENABLE_CXX20_REMOVED_TYPE_TRAITS # NOTE: needed for `std::result_of`
)
target_compile_options(cxx-swift-abi PUBLIC
  -std=gnu++20
  # -stdlib=libc++
  -Werror
  -Wall
  -Wextra
  -Wthread-safety
  -Wthread-safety-beta
  -pedantic
  -Wno-ambiguous-reversed-operator
  -Wno-deprecated-anon-enum-enum-conversion
  -Wno-deprecated-builtins
  -Wno-deprecated-declarations # NOTE: needed for `std::result_of`
  -Wno-dollar-in-identifier-extension
  -Wno-unused-parameter
  -fno-rtti # needed to avoid "undefined reference to `typeinfo for [...]`" errors
)
