use core::pin::Pin;

pub trait AstWalkerConcreteProxy<'ctx> {}

pub struct AstWalkerDynProxy<'state, 'ctx: 'state> {
    pub(crate) _inner: Pin<&'state mut dyn AstWalkerConcreteProxy<'ctx>>,
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        #[cxx_name = ASTWalkerDynProxy]
        type AstWalkerDynProxy<'state, 'ctx>;
    }
}
