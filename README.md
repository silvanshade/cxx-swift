# cxx-swift

Rust [cxx](https://cxx.rs/)-style FFI bindings to the Swift C++ API

### Build Instructions

Building the `cxx-swift` crate is a somewhat involved process since it requires having built the [Swift toolchain](https://github.com/apple/swift), which in turn requires building (or bootstrapping from) LLVM and Clang.

#### Using the devcontainer

The quickest and easiest way to be able to develop and build the `cxx-swift` crate is to use the [devcontainer](https://containers.dev/) definition in the [`.devcontainer`](https://github.com/silvanshade/cxx-swift/tree/main/.devcontainer) directory.

The devcontainer provides a self-contained development environment with the necessary libraries for building the FFI bindings for the Swift toolchain already compiled, along with LLVM, Clang, and the Rust toolchain.

The devcontainer uses a multi-platform image with variants available for both arm64 and amd64, which will be chosen automatically for your platform; the arm64 variant will be chosen for Apple Silicon Macs, so there is no need to run the amd64 variant under Docker with Rosetta.

In order to use the devcontainer, if you open the repo with [`Visual Studio Code`](https://code.visualstudio.com/) and have the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed (you will also need [Docker](https://www.docker.com/) installed), the editor should prompt you to open the workspace in the devcontainer (if not, manually open the command palette and select "Dev Containers: Rebuild and Reopen in Container").

Alternatively, you should be able to open the devcontainer with other [compatible tools](https://containers.dev/supporting), including the [cli](https://github.com/devcontainers/cli) tool.

*NOTE: The devcontainer uses a docker image that is around 2.5GB compressed, which decompresses locally to around 15GB. You can expect the devcontainer initialization process to take a few minutes the very first time you launch it (in which the editor may look like it has stalled, but will eventually finish). In any case, it should still be significantly faster than having to download and compile the entire LLVM, Clang, and Swift toolchains. After the first initialization, subsequently launching the devcontainer should be almost instantaneous.*

#### Building on macOS

1. Start from a common development directory, e.g., `~/Development`
2. `brew install llvm sccache`
3. `mkdir swift-project`
4. `cd swift-project`
5. `git clone --depth 1 --single-branch --branch swift-5.8.1-RELEASE https://github.com/apple/swift.git swift`
6. `cd swift`
7. `./utils/update-checkout --clone --skip-history --skip-tags --tag swift-5.8.1-RELEASE`
8. `./utils/build-script --host-cc /opt/homebrew/opt/llvm@16/bin/clang --host-cxx /opt/homebrew/opt/llvm@16/bin/clang++ --bootstrapping hosttools --build-toolchain-only true --build-swift-private-stdlib false --skip-build-compiler-rt --skip-build-clang-tools-extra --skip-early-swift-driver --skip-early-swiftsyntax --skip-build-benchmarks --enable-experimental-differentiable-programming false --enable-experimental-concurrency false --enable-experimental-distributed false --enable-experimental-string-processing false --sccache --min-size-release --clean --darwin-deployment-version-os "$(otool -l /opt/homebrew/opt/llvm@16/bin/clang++ | awk '/minos/ { print $2 }')" --host-libtool /opt/homebrew/opt/llvm@16/bin/llvm-libtool-darwin --host-lipo /opt/homebrew/opt/llvm@16/bin/llvm-lipo`
9. `cd ~/Development`
10. `git clone https://github.com/silvanshade/cxx-swift`
11. `cd cxx-swift`
12. `CC=/opt/homebrew/opt/llvm@16/bin/clang CXX=/opt/homebrew/opt/llvm@16/bin/clang++ SWIFT_PROJECT_PATH="${HOME}/Development/swift-project" cargo build`

NOTE: the `SWIFT_PROJECT_PATH` environment variable must be set and must be an absolute path pointing to the Swift toolchain built in step (8) when running `cargo` commands since the `cxx-swift` build scripts rely on this variable to locate the necessary LLVM / Clang / Swift toolchain headers and static libraries.

#### Building on Linux

1. Start from a common development directory, e.g., `~/Development`
2. `brew install llvm sccache`
3. `mkdir swift-project`
4. `cd swift-project`
5. `git clone --depth 1 --single-branch --branch swift-5.8.1-RELEASE https://github.com/apple/swift.git swift`
6. `cd swift`
7. `./utils/update-checkout --clone --skip-history --skip-tags --tag swift-5.8.1-RELEASE`
8. `./utils/build-script --host-cc clang-16 --host-cxx clang++-16 --bootstrapping hosttools --build-toolchain-only true --build-swift-private-stdlib false --skip-build-compiler-rt --skip-build-clang-tools-extra --skip-early-swift-driver --skip-early-swiftsyntax --skip-build-benchmarks --enable-experimental-differentiable-programming false --enable-experimental-concurrency false --enable-experimental-distributed false --enable-experimental-string-processing false --sccache --min-size-release --clean`
9. `cd ~/Development`
10. `git clone https://github.com/silvanshade/cxx-swift`
11. `cd cxx-swift`
12. `CC=clang-15 CXX=clang++-16 SWIFT_PROJECT_PATH="${HOME}/Development/swift-project" cargo build`

NOTE: the `SWIFT_PROJECT_PATH` environment variable must be set and must be an absolute path pointing to the Swift toolchain built in step (8) when running `cargo` commands since the `cxx-swift` build scripts rely on this variable to locate the necessary LLVM / Clang / Swift toolchain headers and static libraries.
