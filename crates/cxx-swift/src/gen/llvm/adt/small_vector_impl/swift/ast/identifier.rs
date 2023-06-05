#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-abi/cxx/include/llvm/ADT/SmallVectorImpl/swift/AST/Identifier.hxx");
        include!("cxx-swift-abi/cxx/include/swift/AST/Identifier.hxx");

        #[namespace = "cxx_swift::swift::ast::identifier"]
        type Identifier<'ctx> = crate::ffi::swift::ast::identifier::Identifier<'ctx>;

        #[namespace = "cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier"]
        type SmallVectorImpl<'ctx> =
            crate::ffi::llvm::adt::small_vector_impl::swift::ast::identifier::SmallVectorImpl<'ctx>;
    }

    #[namespace = "cxx_llvm::llvm::adt::small_vector_impl::swift::ast::identifier"]
    unsafe extern "C++" {
        fn as_slice<'this, 'ctx>(This: &'this SmallVectorImpl<'ctx>) -> &'this [Identifier<'ctx>];

        unsafe fn as_mut_slice<'this, 'ctx>(
            This: Pin<&'this mut SmallVectorImpl<'ctx>>,
        ) -> &'this mut [Identifier<'ctx>];
    }
}
pub(crate) use self::ffi::*;
