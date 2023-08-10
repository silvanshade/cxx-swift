use crate::gen::swift::basic::lang_options;
use core::pin::Pin;
use cxx_llvm::llvm::adt::triple::Triple;
use moveref::MoveRef;
use snafu::prelude::*;

pub use crate::auto::swift::basic::lang_options::LangOptions;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("OS was invalid when setting target for `LangOptions`"))]
    SetTargetOsWasInvalid { backtrace: snafu::Backtrace },
    #[snafu(display("Arch was invalid when setting target for `LangOptions`"))]
    SetTargetArchWasInvalid { backtrace: snafu::Backtrace },
}

impl LangOptions {
    #[inline]
    pub fn new() -> impl moveref::New<Output = LangOptions> {
        Self::default_new()
    }
}

impl LangOptions {
    #[inline]
    pub fn set_target(self: Pin<&mut Self>, triple: Pin<MoveRef<'_, Triple>>) -> Result<(), Error> {
        let mut inner = unsafe { Pin::into_inner_unchecked(triple) };
        let ptr = inner.as_mut_ptr();
        let mut os_was_invalid = false;
        let mut arch_was_invalid = false;
        unsafe { lang_options::set_target(self, ptr, &mut os_was_invalid, &mut arch_was_invalid) };
        ensure!(!os_was_invalid, SetTargetOsWasInvalidSnafu);
        ensure!(!arch_was_invalid, SetTargetArchWasInvalidSnafu);
        Ok(())
    }
}
