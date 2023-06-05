#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[repr(C, align(8))]
pub struct ClangImporterOptions {
    _layout: [u8; 296],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _pinned: ::core::marker::PhantomPinned,
}
unsafe impl ::cxx::ExternType for ClangImporterOptions {
    type Id = ::cxx::type_id!("cxx_swift::swift::basic::clang_importer_options::ClangImporterOptions");
    type Kind = ::cxx::kind::Opaque;
}
impl ::core::ops::Drop for ClangImporterOptions {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self::ffi::cxx_destruct(self);
        }
    }
}
#[cfg(feature = "debug")]
impl ::core::fmt::Debug for ClangImporterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ClangImporterOptions").finish()
    }
}
impl ClangImporterOptions {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = ClangImporterOptions> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl ::cxx_memory::CopyNew for ClangImporterOptions {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl ::cxx_memory::MoveNew for ClangImporterOptions {
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
    #[namespace = "cxx_swift::swift::basic::clang_importer_options"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/Basic/ClangImporterOptions.hxx");
        #[cxx_name = "ClangImporterOptions"]
        #[allow(unused)]
        type ClangImporterOptions = super::ClangImporterOptions;
        unsafe fn cxx_copy_new(This: *mut ClangImporterOptions, that: &ClangImporterOptions);
        unsafe fn cxx_move_new(This: *mut ClangImporterOptions, that: *mut ClangImporterOptions);
        unsafe fn cxx_default_new(This: *mut ClangImporterOptions);
        unsafe fn cxx_destruct(This: *mut ClangImporterOptions);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<ClangImporterOptions>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<ClangImporterOptions>(), 296)
        }
    }
}
