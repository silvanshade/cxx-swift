# cxx-swift

### Build instructions for macOS

1. Start from a common development directory, e.g., `~/Development`
2. `brew install llvm sccache`

NOTE: This project requires using the standard Clang toolchain rather than the Apple Clang toolchain for various compatibility reasons, thus the need to install `llvm` via homebrew.

3. `mkdir swift-project`
4. `cd swift-project`
5. `git clone --depth 1 --single-branch --branch swift-5.8.1-RELEASE https://github.com/apple/swift.git swift`
6. `cd swift`
7. `./utils/update-checkout --clone --skip-history --skip-tags --tag swift-5.8.1-RELEASE`
8. `./utils/build-script --host-cc /opt/homebrew/opt/llvm@16/bin/clang --host-cxx /opt/homebrew/opt/llvm@16/bin/clang++ --bootstrapping hosttools --build-toolchain-only true --build-swift-private-stdlib false --skip-build-compiler-rt --skip-build-clang-tools-extra --skip-early-swift-driver --skip-early-swiftsyntax --skip-build-benchmarks --enable-experimental-differentiable-programming false --enable-experimental-concurrency false --enable-experimental-distributed false --enable-experimental-string-processing false --sccache --min-size-release --clean --darwin-deployment-version-os "$(otool -l /opt/homebrew/opt/llvm@16/bin/clang++ | awk '/minos/ { print $2 }')" --host-libtool /opt/homebrew/opt/llvm@16/bin/llvm-libtool-darwin --host-lipo /opt/homebrew/opt/llvm@16/bin/llvm-lipo`
9. `cd ~/Development`
10. `git clone https://github.com/silvanshade/cxx-swift`
11. `cd cxx-swift`
12. `CC=/opt/homebrew/opt/llvm@16/bin/clang CXX=/opt/homebrew/opt/llvm@16/bin/clang++ SWIFT_PROJECT_PATH="${HOME}/Development/swift-project" cargo test`

NOTE: the `SWIFT_PROJECT_PATH` environment variable must be set and must be an absolute path pointing to the Swift toolchain built in step (8) when running `cargo` commands since the `cxx-swift` build scripts rely on this variable to locate the necessary LLVM / Clang / Swift toolchain headers and static libraries.
