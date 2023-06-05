use crate::{ffi::swift::basic::source_manager::SourceManager, gen::swift::ast::diagnostic_engine};
use core::{mem::MaybeUninit, pin::Pin};

pub use crate::abi::swift::ast::diagnostic_engine::DiagnosticEngine;

impl<'source_manager> DiagnosticEngine<'source_manager> {
    #[inline]
    pub fn from<Data>(data: Data) -> impl cxx_memory::New<Output = DiagnosticEngine<'source_manager>>
    where
        Data: Into<crate::Initializer<Self, Data>>,
    {
        data.into()
    }
}

impl<'a, 'source_manager> From<Pin<&'a mut SourceManager>>
    for crate::Initializer<DiagnosticEngine<'source_manager>, Pin<&'a mut SourceManager>>
{
    #[inline]
    fn from(data: Pin<&'a mut SourceManager>) -> Self {
        #[inline]
        unsafe fn closure<'a>(this: Pin<&mut MaybeUninit<DiagnosticEngine>>, data: Pin<&'a mut SourceManager>) {
            let this = this.get_unchecked_mut().as_mut_ptr();
            unsafe { diagnostic_engine::cxx_placement_new_from_source_manager(this, data) }
        }
        crate::Initializer::new(closure, data)
    }
}
