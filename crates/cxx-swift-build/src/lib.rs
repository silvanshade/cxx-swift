use cxx_llvm_build_common::prelude::*;
use snafu::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Expected LLVM project path to be `{expected}` but was `{actual}`"))]
    InvalidLlvmProjectPath { expected: String, actual: String },
    #[snafu(display("Expected LLVM CMake build path to be `{expected}` but was `{actual}`"))]
    InvalidLlvmCmakeBuildPath { expected: String, actual: String },
    #[snafu(display("Expected Clang project path to be `{expected}` but was `{actual}`"))]
    InvalidClangProjectPath { expected: String, actual: String },
    #[snafu(display("Expected Clang CMake build path to be `{expected}` but was `{actual}`"))]
    InvalidClangCmakeBuildPath { expected: String, actual: String },
    #[snafu(display(
        "Failed to locate Cmark CMake build path. Try setting the `CMARK_CMAKE_BUILD_PATH` environment variable."
    ))]
    FailedToLocateCmarkCmakeBuildPath,
    #[snafu(display(
        "Failed to locate Swift project path. Try setting the `SWIFT_PROJECT_PATH` environment variable."
    ))]
    FailedToLocateSwiftProjectPath,
    #[snafu(display(
        "Failed to locate Swift CMake build path. Try setting the `SWIFT_CMAKE_BUILD_PATH` environment variable."
    ))]
    FailedToLocateSwiftCmakeBuildPath,
    #[snafu(display("{source}"))]
    GetPathFromEnv { source: BoxError },
}

pub struct Dirs {
    pub cmark_cmake_build: PathBuf,
    pub swift_project: PathBuf,
    pub swift_cmake_build: PathBuf,
}

fn locate_cmark_cmake_build(swift_project: &Path) -> Result<PathBuf, self::Error> {
    let err = Err(self::Error::FailedToLocateCmarkCmakeBuildPath);

    let mut should_ignore = false;

    if let Some(cmark_cmake_build) =
        get_path_from_env("CMARK_CMAKE_BUILD_PATH", &mut should_ignore).context(GetPathFromEnvSnafu)?
    {
        return Ok(cmark_cmake_build);
    }

    if should_ignore {
        return err;
    }

    let cmark_cmake_build = swift_project.join(PathBuf::from_iter([
        "build",
        &NINJA_BUILD_DIR(),
        &format!("cmark-{CMAKE_BUILD_TARGET}"),
    ]));

    if cmark_cmake_build.exists() {
        return Ok(cmark_cmake_build);
    }

    err
}

fn locate_swift_cmake_build_path(swift_project: &Path) -> Result<PathBuf, self::Error> {
    let err = Err(self::Error::FailedToLocateSwiftCmakeBuildPath);

    let mut should_ignore = false;

    if let Some(swift_cmake_build) =
        get_path_from_env("SWIFT_CMAKE_BUILD_PATH", &mut should_ignore).context(GetPathFromEnvSnafu)?
    {
        return Ok(swift_cmake_build);
    }

    if should_ignore {
        return err;
    }

    let swift_cmake_build = swift_project.join(PathBuf::from_iter([
        "build",
        &NINJA_BUILD_DIR(),
        &format!("swift-{CMAKE_BUILD_TARGET}"),
    ]));

    if swift_cmake_build.exists() {
        return Ok(swift_cmake_build);
    }

    err
}

impl Dirs {
    pub fn new(
        cargo_pkg_name: &str,
        llvm_dirs: &cxx_llvm_build::Dirs,
        clang_dirs: &cxx_clang_build::Dirs,
    ) -> Result<Dirs, self::Error> {
        let cxx_llvm_build::Dirs {
            swift_project,
            llvm_project,
            llvm_cmake_build,
        } = llvm_dirs;

        let cxx_clang_build::Dirs {
            clang_project,
            clang_cmake_build,
        } = clang_dirs;

        if let Some(swift_project) = swift_project {
            let llvm_project_expected = swift_project.join("llvm-project");
            ensure!(llvm_project == &llvm_project_expected, InvalidLlvmProjectPathSnafu {
                expected: llvm_project_expected.display().to_string(),
                actual: llvm_project.display().to_string(),
            });

            let llvm_cmake_build_expected = swift_project.join(PathBuf::from_iter([
                "build",
                &NINJA_BUILD_DIR(),
                &format!("llvm-{CMAKE_BUILD_TARGET}"),
            ]));
            ensure!(
                llvm_cmake_build == &llvm_cmake_build_expected,
                InvalidLlvmCmakeBuildPathSnafu {
                    expected: llvm_cmake_build_expected.display().to_string(),
                    actual: llvm_cmake_build.display().to_string(),
                }
            );

            let clang_project_expected = llvm_project.join("clang");
            ensure!(clang_project == &clang_project_expected, InvalidClangProjectPathSnafu {
                expected: clang_project_expected.display().to_string(),
                actual: clang_project.display().to_string(),
            });

            let clang_cmake_build_expected = llvm_cmake_build.join("tools/clang");
            ensure!(
                clang_cmake_build == &clang_cmake_build_expected,
                InvalidClangCmakeBuildPathSnafu {
                    expected: clang_cmake_build_expected.display().to_string(),
                    actual: clang_cmake_build.display().to_string(),
                }
            );

            let swift_cmake_build = locate_swift_cmake_build_path(swift_project)?;
            println!(
                "cargo:warning=[{cargo_pkg_name}]: Swift CMake build path: \"{}\"",
                swift_cmake_build.display()
            );

            let cmark_cmake_build = locate_cmark_cmake_build(swift_project)?;
            println!(
                "cargo:warning=[{cargo_pkg_name}]: cmark CMake build path: \"{}\"",
                cmark_cmake_build.display()
            );

            return Ok(Dirs {
                cmark_cmake_build,
                swift_project: swift_project.clone(),
                swift_cmake_build,
            });
        }

        Err(Error::FailedToLocateSwiftProjectPath)
    }
}

pub fn cxx_build(
    llvm_dirs: &cxx_llvm_build::Dirs,
    clang_dirs: &cxx_clang_build::Dirs,
    swift_dirs: &Dirs,
    rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>,
    files: impl IntoIterator<Item = impl AsRef<Path>>,
    output: &str,
) -> BoxResult<()> {
    rustc_link_searches(llvm_dirs, clang_dirs, swift_dirs);
    cxx_build::bridges(rust_source_files)
        .llvm_common_compiler()
        .llvm_common_defines()
        .llvm_common_flags()
        .files(files)
        .try_compile(output)?;
    rustc_link_libs();
    Ok(())
}

pub fn rustc_link_searches(llvm_dirs: &cxx_llvm_build::Dirs, clang_dirs: &cxx_clang_build::Dirs, swift_dirs: &Dirs) {
    println!(
        "cargo:rustc-link-search={}",
        swift_dirs.cmark_cmake_build.join("src").display()
    );
    cxx_llvm_build::rustc_link_searches(llvm_dirs);
    cxx_clang_build::rustc_link_searches(llvm_dirs, clang_dirs);
    println!(
        "cargo:rustc-link-search={}",
        swift_dirs.swift_cmake_build.join("lib").display()
    );
}

pub fn rustc_link_libs() {
    println!("cargo:rustc-link-lib=static=swiftClangImporter");
    println!("cargo:rustc-link-lib=static=swiftParse");
    println!("cargo:rustc-link-lib=static=swiftAST");
    println!("cargo:rustc-link-lib=static=swiftDemangling");
    println!("cargo:rustc-link-lib=static=swiftParse");
    println!("cargo:rustc-link-lib=static=swiftMarkup");
    println!("cargo:rustc-link-lib=static=swiftBasic");
    cxx_clang_build::rustc_link_libs();
    cxx_llvm_build::rustc_link_libs();
    println!("cargo:rustc-link-lib=static=cmark-gfm");
    println!("cargo:rustc-link-lib=uuid");
}
