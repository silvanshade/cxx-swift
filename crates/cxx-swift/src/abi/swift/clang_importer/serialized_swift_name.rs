#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub(crate) mod small_vector;
pub(crate) mod small_vector_boxed;
pub(crate) mod small_vector_impl;
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct SerializedSwiftName<'ctx> {
    _layout: [u8; 24],
    _neither_send_nor_sync: ::core::marker::PhantomData<[*const u8; 0]>,
    _lifetimes: ::core::marker::PhantomData<(&'ctx (),)>,
}
unsafe impl<'ctx> ::cxx::ExternType for SerializedSwiftName<'ctx> {
    type Id = ::cxx::type_id!("cxx_swift::swift::clang_importer::serialized_swift_name::SerializedSwiftName");
    type Kind = ::cxx::kind::Trivial;
}
#[cfg(feature = "debug")]
impl<'ctx> ::core::fmt::Debug for SerializedSwiftName<'ctx> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SerializedSwiftName").finish()
    }
}
impl<'ctx> SerializedSwiftName<'ctx> {
    #[inline]
    pub fn default() -> impl ::cxx_memory::New<Output = SerializedSwiftName<'ctx>> {
        unsafe {
            ::cxx_memory::new::by_raw(move |this| {
                let this = this.get_unchecked_mut().as_mut_ptr();
                self::ffi::cxx_default_new(this);
            })
        }
    }
}
unsafe impl<'ctx> ::cxx_memory::CopyNew for SerializedSwiftName<'ctx> {
    #[inline]
    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
        let this = this.get_unchecked_mut().as_mut_ptr();
        self::ffi::cxx_copy_new(this, that)
    }
}
unsafe impl<'ctx> ::cxx_memory::MoveNew for SerializedSwiftName<'ctx> {
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
    #[namespace = "cxx_swift::swift::clang_importer::serialized_swift_name"]
    unsafe extern "C++" {
        include!("cxx-swift-abi/cxx/include/swift/ClangImporter/SerializedSwiftName.hxx");
        #[cxx_name = "SerializedSwiftName"]
        #[allow(unused)]
        type SerializedSwiftName<'ctx> = super::SerializedSwiftName<'ctx>;
        unsafe fn cxx_copy_new<'ctx>(This: *mut SerializedSwiftName<'ctx>, that: &SerializedSwiftName<'ctx>);
        unsafe fn cxx_move_new<'ctx>(This: *mut SerializedSwiftName<'ctx>, that: *mut SerializedSwiftName<'ctx>);
        unsafe fn cxx_default_new<'ctx>(This: *mut SerializedSwiftName<'ctx>);
    }
}
#[cfg(test)]
mod info {
    use super::*;
    mod test {
        use super::*;
        #[test]
        fn cxx_abi_align() {
            ::core::assert_eq!(::core::mem::align_of::<SerializedSwiftName<'static>>(), 8)
        }
        #[test]
        fn cxx_abi_size() {
            ::core::assert_eq!(::core::mem::size_of::<SerializedSwiftName<'static>>(), 24)
        }
        :: static_assertions :: assert_impl_all ! (SerializedSwiftName < 'static > : :: core :: marker :: Copy);
        :: static_assertions :: assert_impl_all ! (SerializedSwiftName < 'static > : :: core :: marker :: Unpin);
    }
}
