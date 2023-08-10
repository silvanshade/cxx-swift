# Docker Image Build Instructions

For deployment of the multi-platform images, we recommend building on a Apple Silicon host machine. This way, the `amd64` images can use Rosetta 2 for emulation (if enabled in Docker), which is nearly as fast as native. In contrast, if you were to build the images on an `amd64` host machine, the `arm64` images would have to use the default QEMU emulation layer, which is drastically slower than native.

If you intend only to run the images locally for a single platform, then it does not matter which host architecture you use to build the images and you can modify the `--platform` flag parameters appropriately in the steps below. Similarly, you can skip the individual platform-specific steps for the other architecture.

Execute the following commands from the project root directory (`../docker/..`):

1. `docker buildx build --platform linux/amd64,linux/arm64 -t base-clang -f ./docker/Dockerfile.base-clang .`
2. `docker buildx build --platform linux/amd64,linux/arm64 -t base-rust -f ./docker/Dockerfile.base-rust .`
3. `docker buildx build --platform linux/amd64,linux/arm64 -t base-swift-project -f ./docker/Dockerfile.base-swift-project .`
4.
    * `docker buildx build --platform linux/amd64 -t base-swift-project-tools:amd64 -f ./docker/Dockerfile.base-swift-project-tools.amd64 .`
    * `docker buildx build --platform linux/arm64 -t base-swift-project-tools:arm64 -f ./docker/Dockerfile.base-swift-project-tools.arm64 .`
5. `docker buildx build --platform linux/amd64,linux/arm64 -t base-swift-project-compiled -f ./docker/Dockerfile.base-swift-project-compiled .`
6.
    * `docker buildx build --platform linux/amd64 -t base-swift-project-artifacts:amd64 -f ./docker/Dockerfile.base-swift-project-artifacts.amd64 .`
    * `docker buildx build --platform linux/arm64 -t base-swift-project-artifacts:arm64 -f ./docker/Dockerfile.base-swift-project-artifacts.arm64 .`
7. `docker buildx build --platform linux/amd64,linux/arm64 -t cxx-swift -f ./docker/Dockerfile.cxx-swift .`
8. `docker buildx build --platform linux/amd64,linux/arm64 -t cxx-swift-dev -f ./docker/Dockerfile.cxx-swift-dev .`
