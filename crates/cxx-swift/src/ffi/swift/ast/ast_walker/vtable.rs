use crate::{ffi::swift::ast::ast_walker_base::AstWalkerBase, util::class::Vtable};
use core::{ops::DerefMut, pin::Pin};

pub trait AstWalkerConcrete<'ctx>: private::Sealed + crate::TracingDebug {
    fn base_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>>;
    fn prev_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>>;
    fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool;
    fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool;
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool;
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool;
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool;
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool;
}

pub trait AstWalkerAbstract<'ctx, Concrete>
where
    Concrete: AstWalkerConcrete<'ctx>,
    Self: crate::TracingDebug,
{
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_generic_params(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_into_generic_params()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_tap_expression(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_into_tap_expression()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_into_property_wrapper_placeholder_value()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_capture_initializer_expressions()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_accessors_the_old_way()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>, prev: Pin<&mut Concrete>) -> bool {
        prev.should_walk_serialized_top_level_internal_decls()
    }
}

impl<'ctx> AstWalkerConcrete<'ctx> for AstWalkerBase<'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn base_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn prev_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_into_generic_params(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_into_tap_expression(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_into_property_wrapper_placeholder_value(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_capture_initializer_expressions(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_accessors_the_old_way(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        AstWalkerBase::should_walk_serialized_top_level_internal_decls(self)
    }
}

impl<'ctx, Abstract, Concrete> AstWalkerConcrete<'ctx> for Vtable<Abstract, Concrete>
where
    Abstract: AstWalkerAbstract<'ctx, Concrete>,
    Concrete: AstWalkerConcrete<'ctx>,
{
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn base_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        let pin = self.project();
        pin.prev.base_mut()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn prev_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        let pin = self.project();
        pin.prev
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_into_generic_params(pin.prev)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_into_tap_expression(pin.prev)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_into_property_wrapper_placeholder_value(pin.prev)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_capture_initializer_expressions(pin.prev)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_accessors_the_old_way(pin.prev)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        let pin = self.project();
        pin.this.should_walk_serialized_top_level_internal_decls(pin.prev)
    }
}

impl<'ctx, Concrete: DerefMut + Unpin> AstWalkerConcrete<'ctx> for Pin<Concrete>
where
    Concrete: crate::TracingDebug,
    Concrete::Target: AstWalkerConcrete<'ctx>,
{
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn base_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        self.get_mut().as_mut().base_mut()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn prev_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        self.get_mut().as_mut().prev_mut()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        self.get_mut().as_mut().should_walk_into_generic_params()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        self.get_mut().as_mut().should_walk_into_tap_expression()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        self.get_mut()
            .as_mut()
            .should_walk_into_property_wrapper_placeholder_value()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        self.get_mut().as_mut().should_walk_capture_initializer_expressions()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        self.get_mut().as_mut().should_walk_accessors_the_old_way()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        self.get_mut()
            .as_mut()
            .should_walk_serialized_top_level_internal_decls()
    }
}

impl<'ctx, Concrete> AstWalkerConcrete<'ctx> for Box<Concrete>
where
    Concrete: AstWalkerConcrete<'ctx>,
{
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn base_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.base_mut()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn prev_mut(self: Pin<&mut Self>) -> Pin<&mut dyn AstWalkerConcrete<'ctx>> {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.prev_mut()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_generic_params(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_into_generic_params()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_tap_expression(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_into_tap_expression()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_into_property_wrapper_placeholder_value(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_into_property_wrapper_placeholder_value()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_capture_initializer_expressions(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_capture_initializer_expressions()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_accessors_the_old_way(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_accessors_the_old_way()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn should_walk_serialized_top_level_internal_decls(self: Pin<&mut Self>) -> bool {
        let this = unsafe { self.map_unchecked_mut(|x| x.as_mut()) };
        this.should_walk_serialized_top_level_internal_decls()
    }
}

pub struct AstWalkerDyn<'state, 'ctx: 'state> {
    pub(crate) inner: Pin<&'state mut dyn AstWalkerConcrete<'ctx>>,
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for AstWalkerDyn<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AstWalkerDyn").finish()
    }
}

impl<'state, 'ctx: 'state> AstWalkerDyn<'state, 'ctx> {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_into_generic_params(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().should_walk_into_generic_params()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_into_tap_expression(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().should_walk_into_tap_expression()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_into_property_wrapper_placeholder_value(mut self: Pin<&mut Self>) -> bool {
        self.inner
            .as_mut()
            .should_walk_into_property_wrapper_placeholder_value()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_capture_initializer_expressions(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().should_walk_capture_initializer_expressions()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_accessors_the_old_way(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().should_walk_accessors_the_old_way()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    pub(crate) fn should_walk_serialized_top_level_internal_decls(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().should_walk_serialized_top_level_internal_decls()
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl<'ctx> Sealed for super::AstWalkerBase<'ctx> {
    }

    impl<'ctx, Abstract, Concrete> Sealed for super::Vtable<Abstract, Concrete>
    where
        Abstract: AstWalkerAbstract<'ctx, Concrete>,
        Concrete: AstWalkerConcrete<'ctx>,
    {
    }

    impl<P: DerefMut + Unpin> Sealed for Pin<P> where P::Target: Sealed
    {
    }

    impl<T> Sealed for Box<T> where T: Sealed
    {
    }
}
