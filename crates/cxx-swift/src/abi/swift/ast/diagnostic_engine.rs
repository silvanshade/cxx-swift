#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct DiagnosticEngine<'source_manager> {
    _layout: [u8; 2408],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
    _lifetimes: ::core::marker::PhantomData<(&'source_manager (),)>,
}
unsafe impl<'source_manager> ::cxx::ExternType for DiagnosticEngine<'source_manager> {
    type Id = ::cxx::type_id!("cxx_swift::swift::ast::diagnostic_engine::DiagnosticEngine");
    type Kind = ::cxx::kind::Opaque;
}
impl<'source_manager> ::core::ops::Drop for DiagnosticEngine<'source_manager> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl<'source_manager> ::core::fmt::Debug for DiagnosticEngine<'source_manager> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DiagnosticEngine").finish()
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::ast::diagnostic_engine"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/AST/DiagnosticEngine.hxx");
        #[cxx_name = "DiagnosticEngine"]
        #[allow(unused)]
        type DiagnosticEngine<'source_manager> = super::DiagnosticEngine<'source_manager>;
        unsafe fn cxx_destruct<'source_manager>(This: *mut DiagnosticEngine<'source_manager>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<DiagnosticEngine<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<DiagnosticEngine<'static>>(), 2408)
        }
    }
}
