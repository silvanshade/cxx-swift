use crate::common::*;
use core::{
    ops::{Deref, DerefMut},
    pin::{pin, Pin},
};
use swift::ast::ast_walker::{
    vtable::{AstWalkerAbstract, AstWalkerConcrete},
    AstWalkerBase,
    AstWalkerRust,
};
#[cfg(feature = "tracing")]
use tracing_subscriber::prelude::*;
use util::class::Vtable;

#[test]
fn test() -> BoxResult<()> {
    #[cfg(feature = "tracing")]
    tracing_subscriber::registry()
        .with(tracing_forest::ForestLayer::default())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()?;

    #[derive(Default, Debug)]
    struct Abstract00 {
        calls_from_next00: Vec<String>,
        calls_from_next01: Vec<String>,
        calls_from_next02: Vec<String>,
    }
    impl<'ctx, Concrete00> AstWalkerAbstract<'ctx, Concrete00> for Abstract00
    where
        Concrete00: AstWalkerConcrete<'ctx>,
        Concrete00: Deref<Target = AstWalkerBase<'ctx>>,
    {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        #[inline]
        fn should_walk_into_generic_params(mut self: Pin<&mut Self>, prev: Pin<&mut Concrete00>) -> bool {
            let str = "Next00::should_walk_into_generic_params";
            self.calls_from_next00.push(str.into());
            prev.should_walk_into_generic_params()
        }
    }

    #[derive(Default, Debug)]
    struct Abstract01;
    impl<'ctx, Concrete00, Concrete01> AstWalkerAbstract<'ctx, Concrete01> for Abstract01
    where
        Concrete01: AstWalkerConcrete<'ctx>,
        Concrete01: DerefMut<Target = Vtable<Abstract00, Concrete00>>,
        Concrete01: Unpin,
    {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        #[inline]
        fn should_walk_into_generic_params(self: Pin<&mut Self>, mut prev: Pin<&mut Concrete01>) -> bool {
            let str = "Next01::should_walk_into_generic_params";
            prev.this.calls_from_next01.push(str.into());
            prev.should_walk_into_generic_params()
        }

        #[cfg_attr(feature = "tracing", tracing::instrument)]
        #[inline]
        fn should_walk_into_tap_expression(self: Pin<&mut Self>, mut prev: Pin<&mut Concrete01>) -> bool {
            let str = "Next01::should_walk_into_tap_expression";
            prev.this.calls_from_next01.push(str.into());
            prev.should_walk_into_tap_expression()
        }
    }

    #[derive(Default, Debug)]
    struct Abstract02;
    impl<'ctx, Concrete00, Concrete01, Concrete02> AstWalkerAbstract<'ctx, Concrete02> for Abstract02
    where
        Concrete01: AstWalkerConcrete<'ctx>,
        Concrete01: DerefMut<Target = Vtable<Abstract00, Concrete00>>,
        Concrete01: Unpin,
        Concrete02: AstWalkerConcrete<'ctx>,
        Concrete02: DerefMut<Target = Vtable<Abstract01, Concrete01>>,
        Concrete02: Unpin,
    {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        #[inline]
        fn should_walk_into_tap_expression(self: Pin<&mut Self>, mut prev: Pin<&mut Concrete02>) -> bool {
            let str = "Next02::should_walk_into_tap_expression";
            prev.prev.this.calls_from_next02.push(str.into());
            prev.base_mut().should_walk_into_tap_expression()
        }
    }

    #[derive(Default, Debug)]
    struct Abstract03;
    impl<'ctx, Concrete01, Concrete02, Concrete03> AstWalkerAbstract<'ctx, Concrete03> for Abstract03
    where
        Concrete01: AstWalkerConcrete<'ctx>,
        Concrete01: DerefMut<Target = Vtable<Abstract00, AstWalkerBase<'ctx>>>,
        Concrete01: Unpin,
        Concrete02: AstWalkerConcrete<'ctx>,
        Concrete02: DerefMut<Target = Vtable<Abstract01, Concrete01>>,
        Concrete02: Unpin,
        Concrete03: AstWalkerConcrete<'ctx>,
        Concrete03: DerefMut<Target = Vtable<Abstract02, Concrete02>>,
        Concrete03: Unpin,
    {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        #[inline]
        fn should_walk_into_tap_expression(self: Pin<&mut Self>, mut prev: Pin<&mut Concrete03>) -> bool {
            let str = "Next02::should_walk_into_tap_expression";
            prev.prev.prev.this.calls_from_next02.push(str.into());
            prev.base_mut().should_walk_into_tap_expression()
        }
    }

    let_cxx!(mut prev = unsafe { AstWalkerBase::new() });

    let prev = prev.as_mut();
    let this = Abstract00::default();
    let chain = Box::pin(Vtable { this, prev });

    let prev = chain;
    let this = Abstract01::default();
    let chain = pin!(Vtable { this, prev });

    // chain.as_mut().should_walk_into_generic_params();

    // println!();

    // chain.as_mut().should_walk_into_tap_expression();

    // println!();

    let prev = chain;
    let this = Abstract02::default();
    let chain = pin!(Vtable { this, prev });

    // chain.as_mut().should_walk_into_generic_params();

    // println!();

    // chain.as_mut().should_walk_into_tap_expression();

    let_cxx!(mut ast_walker_rust = unsafe { AstWalkerRust::new(chain) });

    println!("success");

    let mut ast_walker = ast_walker_rust.as_mut().deref_pin();
    unsafe {
        ast_walker.as_mut().should_walk_into_generic_params();
        ast_walker.as_mut().should_walk_into_tap_expression();
        ast_walker
            .as_mut()
            .should_walk_into_property_wrapper_placeholder_value();
        ast_walker.as_mut().should_walk_capture_initializer_expressions();
        ast_walker.as_mut().should_walk_accessors_the_old_way();
        ast_walker.as_mut().should_walk_serialized_top_level_internal_decls();
    }

    Ok(())
}
