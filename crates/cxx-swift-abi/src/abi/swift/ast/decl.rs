#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[cxx::bridge]
mod ffi {
    #[namespace = "cxx_swift::swift::ast::decl"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/Decl.hxx");
        fn cxx_abi_align() -> usize;
        fn cxx_abi_size() -> usize;
        fn cxx_is_copy_constructible() -> bool;
        fn cxx_is_move_constructible() -> bool;
        fn cxx_is_default_constructible() -> bool;
        fn cxx_is_destructible() -> bool;
        fn cxx_is_trivially_copyable() -> bool;
        fn cxx_is_trivially_movable() -> bool;
        fn cxx_is_trivially_destructible() -> bool;
        fn rust_should_impl_cxx_extern_type_trivial() -> bool;
        fn rust_should_impl_unpin() -> bool;
        fn rust_should_impl_send() -> bool;
        fn rust_should_impl_sync() -> bool;
        fn rust_should_impl_drop() -> bool;
        fn rust_should_impl_copy() -> bool;
        fn rust_should_impl_default() -> bool;
        fn rust_should_impl_cxx_memory_copy_new() -> bool;
        fn rust_should_impl_cxx_memory_move_new() -> bool;
    }
}
pub use ffi::*;
fn artifact_info() -> ::cxx_memory_abi::CxxAbiArtifactInfo {
    let path_components = vec!["swift", "ast", "decl"];
    let path_descendants = vec![];
    let cxx_include = "cxx-swift-abi/cxx/include/swift/AST/Decl.hxx";
    let cxx_namespace = "cxx_swift::swift::ast::decl";
    let cxx_name = "Decl";
    let rust_name = "Decl";
    let lifetimes = ::cxx_memory_abi::indexmap::IndexMap::from_iter([("ctx", vec![])].into_iter());
    let align = self::ffi::cxx_abi_align();
    let size = self::ffi::cxx_abi_size();
    let is_rust_cxx_extern_type_trivial = {
        let cxx_is_trivially_movable = self::ffi::cxx_is_trivially_movable();
        let rust_should_impl_cxx_extern_type_trivial = self::ffi::rust_should_impl_cxx_extern_type_trivial();
        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
            cxx_is_trivially_movable
        } else {
            rust_should_impl_cxx_extern_type_trivial
        }
    };
    let is_rust_unpin = self::ffi::rust_should_impl_unpin();
    let is_rust_send = self::ffi::rust_should_impl_send();
    let is_rust_sync = self::ffi::rust_should_impl_sync();
    let is_rust_copy = self::ffi::rust_should_impl_copy();
    let is_rust_drop = self::ffi::rust_should_impl_drop();
    let is_rust_default = self::ffi::rust_should_impl_default();
    let is_rust_copy_new = self::ffi::rust_should_impl_cxx_memory_copy_new();
    let is_rust_move_new = self::ffi::rust_should_impl_cxx_memory_move_new();
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
        is_rust_cxx_extern_type_trivial,
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
