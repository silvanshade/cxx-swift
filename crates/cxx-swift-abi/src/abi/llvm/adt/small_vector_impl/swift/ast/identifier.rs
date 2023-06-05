#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[cxx::bridge]
mod ffi {
    #[namespace = "cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/llvm/ADT/SmallVectorImpl/swift/AST/Identifier.hxx");
        fn cxx_abi_align() -> usize;
        fn cxx_abi_size() -> usize;
        fn cxx_is_copy_constructible() -> bool;
        fn cxx_is_move_constructible() -> bool;
        fn cxx_is_default_constructible() -> bool;
        fn cxx_is_destructible() -> bool;
        fn cxx_is_trivially_copyable() -> bool;
        fn cxx_is_trivially_movable() -> bool;
        fn cxx_is_trivially_destructible() -> bool;
    }
}
pub use ffi::*;
fn artifact_info() -> ::cxx_memory_abi::CxxAbiArtifactInfo {
    let path_components = vec!["llvm", "adt", "small_vector_impl", "swift", "ast", "identifier"];
    let path_descendants = vec![];
    let cxx_include = "cxx-swift-abi/cxx/include/llvm/ADT/SmallVectorImpl/swift/AST/Identifier.hxx";
    let cxx_namespace = "cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier";
    let cxx_name = "SmallVectorImpl";
    let rust_name = "SmallVectorImpl";
    let lifetimes = ::cxx_memory_abi::indexmap::IndexMap::from_iter([("ctx", vec![])].into_iter());
    let align = self::ffi::cxx_abi_align();
    let size = self::ffi::cxx_abi_size();
    let is_cxx_trivial = self::ffi::cxx_is_trivially_movable();
    let is_cxx_copy_constructible = self::ffi::cxx_is_copy_constructible();
    let is_cxx_move_constructible = self::ffi::cxx_is_move_constructible();
    let is_cxx_default_constructible = self::ffi::cxx_is_default_constructible();
    let is_cxx_destructible = self::ffi::cxx_is_destructible();
    let is_cxx_trivially_copyable = self::ffi::cxx_is_trivially_copyable();
    let is_cxx_trivially_movable = self::ffi::cxx_is_trivially_movable();
    let is_cxx_trivially_destructible = self::ffi::cxx_is_trivially_destructible();
    let is_rust_unpin = self::ffi::cxx_is_trivially_movable();
    let is_rust_send = false;
    let is_rust_sync = false;
    let is_rust_drop = true && (self::ffi::cxx_is_destructible() && !self::ffi::cxx_is_trivially_destructible());
    let is_rust_default = true;
    let is_rust_copy = self::ffi::cxx_is_trivially_copyable() && self::ffi::cxx_is_trivially_movable() && !is_rust_drop;
    let is_rust_copy_new = true;
    let is_rust_move_new = true;
    ::cxx_memory_abi::CxxAbiArtifactInfo {
        path_components,
        path_descendants,
        cxx_include,
        cxx_namespace,
        cxx_name,
        rust_name,
        lifetimes,
        align,
        size,
        is_cxx_trivial,
        is_cxx_copy_constructible,
        is_cxx_move_constructible,
        is_cxx_default_constructible,
        is_cxx_destructible,
        is_cxx_trivially_copyable,
        is_cxx_trivially_movable,
        is_cxx_trivially_destructible,
        is_rust_unpin,
        is_rust_send,
        is_rust_sync,
        is_rust_copy,
        is_rust_default,
        is_rust_drop,
        is_rust_copy_new,
        is_rust_move_new,
    }
}
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    self::artifact_info().write_module_for_file()
}
