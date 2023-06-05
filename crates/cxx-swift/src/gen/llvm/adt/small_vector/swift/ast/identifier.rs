#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/llvm/ADT/SmallVector/swift/AST/Identifier.hxx");
        include!("cxx-swift-abi/cxx/include/llvm/ADT/SmallVectorImpl/swift/AST/Identifier.hxx");

        #[namespace = "cxx_llvm::llvm::adt::small_vector::swift::ast::identifier"]
        type SmallVector<'ctx> = crate::ffi::llvm::adt::small_vector::swift::ast::identifier::SmallVector<'ctx>;

        #[namespace = "cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::llvm::adt::small_vector_impl::swift::ast::identifier::SmallVectorImpl<'ctx>;
    }

    #[namespace = "cxx_llvm::llvm::adt::small_vector::swift::ast::identifier"]
    unsafe extern "C++" {
        fn as_ref_small_vector_impl<'this, 'ctx>(This: &'this SmallVector<'ctx>) -> &'this SmallVectorImpl<'ctx>;

        fn as_pin_small_vector_impl<'this, 'ctx>(
            This: Pin<&'this mut SmallVector<'ctx>>,
        ) -> Pin<&'this mut SmallVectorImpl<'ctx>>;
    }
}
pub(crate) use self::ffi::*;
