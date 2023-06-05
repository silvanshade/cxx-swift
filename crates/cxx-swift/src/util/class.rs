use pin_project_lite::pin_project;

pin_project! {
pub struct Vtable<Abstract, Concrete> {
    #[pin]
    pub this: Abstract,
    #[pin]
    pub prev: Concrete,
}
}

#[cfg(feature = "debug")]
impl<Abstract, Concrete> core::fmt::Debug for Vtable<Abstract, Concrete>
where
    Abstract: core::fmt::Debug,
    Concrete: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vtable")
            .field("this", &self.this)
            .field("prev", &self.prev)
            .finish()
    }
}
