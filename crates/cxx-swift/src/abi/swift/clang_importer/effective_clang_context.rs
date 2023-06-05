#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct EffectiveClangContext {
    _layout: [u8; 16],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
}
unsafe impl ::cxx::ExternType for EffectiveClangContext {
    type Id = ::cxx::type_id!("cxx_swift::swift::clang_importer::effective_clang_context::EffectiveClangContext");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl ::core::fmt::Debug for EffectiveClangContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EffectiveClangContext").finish()
    }
}
impl EffectiveClangContext {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = EffectiveClangContext> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl ::cxx_memory::CopyNew for EffectiveClangContext {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl ::cxx_memory::MoveNew for EffectiveClangContext {
    #[inline]
    unsafe fn move_new(
        that: ::core::pin::Pin<::cxx_memory::MoveRef<'_, Self>>,
        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
    ) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
        self::ffi::cxx_move_new(this, that)
    }
}
#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "cxx_swift::swift::clang_importer::effective_clang_context"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/EffectiveClangContext.hxx");
        #[cxx_name = "EffectiveClangContext"]
        #[allow(unused)]
        type EffectiveClangContext = super::EffectiveClangContext;
        unsafe fn cxx_copy_new(This: *mut EffectiveClangContext, that: &EffectiveClangContext);
        unsafe fn cxx_move_new(This: *mut EffectiveClangContext, that: *mut EffectiveClangContext);
        unsafe fn cxx_default_new(This: *mut EffectiveClangContext);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<EffectiveClangContext>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<EffectiveClangContext>(), 16)
        }
        :: static_assertions :: assert_impl_all ! (EffectiveClangContext : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (EffectiveClangContext : :: core :: marker :: Unpin);
    }
}
