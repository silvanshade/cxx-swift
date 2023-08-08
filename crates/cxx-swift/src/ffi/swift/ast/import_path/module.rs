use crate::auto::swift::ast::import_path::module;

pub(crate) mod builder;

pub use crate::auto::swift::ast::import_path::module::Module;

impl<'ctx> Module<'ctx> {
    #[inline]
    pub fn less_than(&self, other: &Module<'ctx>) -> bool {
        module::ffi::cxx_operator_less_than(self, other)
    }
}
