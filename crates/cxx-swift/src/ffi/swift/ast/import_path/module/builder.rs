use crate::{
    ffi::swift::ast::{identifier::Identifier, import_path::module::Module as ImportPathModule},
    gen::swift::ast::import_path::module::builder,
};
use core::{mem::MaybeUninit, pin::Pin};

pub use crate::abi::swift::ast::import_path::module::builder::Builder;

impl<'ctx> Builder<'ctx> {
    #[inline]
    pub fn from<Data>(data: Data) -> impl cxx_memory::New<Output = Builder<'ctx>>
    where
        Data: Into<crate::Initializer<Self, Data>>,
    {
        data.into()
    }

    #[inline]
    pub fn get(&self) -> ImportPathModule<'ctx> {
        builder::get(self)
    }
}

impl<'ctx> From<Identifier<'ctx>> for crate::Initializer<Builder<'ctx>, Identifier<'ctx>> {
    #[inline]
    fn from(data: Identifier<'ctx>) -> Self {
        #[inline]
        unsafe fn closure<'ctx>(this: Pin<&mut MaybeUninit<Builder<'ctx>>>, data: Identifier<'ctx>) {
            let this = this.get_unchecked_mut().as_mut_ptr();
            unsafe { builder::cxx_placement_new_from_identifier(this, data) }
        }
        crate::Initializer::new(closure, data)
    }
}
