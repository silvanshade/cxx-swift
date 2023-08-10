#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-swift-auto/cxx/include/swift/AST/Identifier.hxx");
        include!("cxx-swift-auto/cxx/include/swift/AST/Identifier/SmallVectorImpl.hxx");

        #[namespace = "cxx_swift::swift::ast::identifier"]
        type Identifier<'ctx> = crate::ffi::swift::ast::identifier::Identifier<'ctx>;

        #[namespace = "cxx_swift::swift::ast::identifier::small_vector_impl"]
        type SmallVectorImpl<'ctx> = crate::ffi::swift::ast::identifier::small_vector_impl::SmallVectorImpl<'ctx>;
    }

    #[namespace = "cxx_swift::swift::ast::identifier::small_vector_impl"]
    unsafe extern "C++" {
        fn as_slice<'this, 'ctx>(This: &'this SmallVectorImpl<'ctx>) -> &'this [Identifier<'ctx>];

        // NOTE: returning `Pin<&mut [Identifier<'ctx>]>` is currently unsupported (hence the `unsafe`).
        unsafe fn as_mut_slice<'this, 'ctx>(
            This: Pin<&'this mut SmallVectorImpl<'ctx>>,
        ) -> &'this mut [Identifier<'ctx>];
    }
}
pub(crate) use self::ffi::*;
