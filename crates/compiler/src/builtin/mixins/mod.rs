// A reference to the parser is only necessary for some functions
#![allow(unused_variables)]
pub mod meta;

use once_cell::sync::Lazy;

use crate::{ast::ArgumentResult, error::SassResult, evaluate::Visitor};

use std::{
    collections::HashMap,
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

// todo: maybe Identifier instead of str?
pub(crate) type GlobalMixinMap = HashMap<&'static str, BuiltinMixin>;

static MIXIN_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct BuiltinMixin(
    pub(crate) fn(ArgumentResult, &mut Visitor) -> SassResult<()>,
    usize,
);

impl fmt::Debug for BuiltinMixin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builtin")
            .field("id", &self.1)
            .field("fn_ptr", &(self.0 as usize))
            .finish()
    }
}

impl BuiltinMixin {
    pub fn new(body: fn(ArgumentResult, &mut Visitor) -> SassResult<()>) -> BuiltinMixin {
        let count = MIXIN_COUNT.fetch_add(1, Ordering::Relaxed);
        Self(body, count)
    }
}

impl PartialEq for BuiltinMixin {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for BuiltinMixin {}

pub(crate) static GLOBAL_MIXINS: Lazy<GlobalMixinMap> = Lazy::new(|| {
    let mut m = HashMap::new();
    meta::declare(&mut m);
    m
});
