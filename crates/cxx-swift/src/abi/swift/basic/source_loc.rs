#![doc = r" NOTE: This module is auto-generated and should not be edited."]
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct SourceLoc {
    _layout: [u8; 8],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
}
unsafe impl ::cxx::ExternType for SourceLoc {
    type Id = ::cxx::type_id!("cxx_swift::swift::basic::source_loc::SourceLoc");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl ::core::fmt::Debug for SourceLoc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SourceLoc").finish()
    }
}
impl SourceLoc {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = SourceLoc> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl ::cxx_memory::CopyNew for SourceLoc {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl ::cxx_memory::MoveNew for SourceLoc {
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
    #[namespace = "cxx_swift::swift::basic::source_loc"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/Basic/SourceLoc.hxx");
        #[cxx_name = "SourceLoc"]
        #[allow(unused)]
        type SourceLoc = super::SourceLoc;
        unsafe fn cxx_copy_new(This: *mut SourceLoc, that: &SourceLoc);
        unsafe fn cxx_move_new(This: *mut SourceLoc, that: *mut SourceLoc);
        unsafe fn cxx_default_new(This: *mut SourceLoc);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<SourceLoc>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<SourceLoc>(), 8)
        }
        :: static_assertions :: assert_impl_all ! (SourceLoc : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (SourceLoc : :: core :: marker :: Unpin);
    }
}
