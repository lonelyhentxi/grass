mod functions;
pub(crate) mod modules;
mod mixins;

pub(crate) use functions::{
    color, list, map, math, meta, selector, string, DISALLOWED_PLAIN_CSS_FUNCTION_NAMES,
    GLOBAL_FUNCTIONS,
};

#[allow(unused_imports)]
pub(crate) use mixins::{
    GLOBAL_MIXINS
};

pub use functions::Builtin;
pub use mixins::BuiltinMixin;

/// Imports common to all builtin fns
mod builtin_imports {
    pub(crate) use super::functions::{Builtin, GlobalFunctionMap, GLOBAL_FUNCTIONS};
    #[allow(unused_imports)]
    pub(crate) use super::mixins::{BuiltinMixin, GlobalMixinMap, GLOBAL_MIXINS};

    pub(crate) use codemap::{Span, Spanned};

    #[cfg(feature = "random")]
    pub(crate) use rand::{distributions::Alphanumeric, thread_rng, Rng};

    pub(crate) use crate::{
        ast::{Argument, ArgumentDeclaration, ArgumentResult, MaybeEvaledArguments},
        color::Color,
        common::{BinaryOp, Brackets, Identifier, ListSeparator, QuoteKind},
        error::SassResult,
        evaluate::Visitor,
        unit::Unit,
        value::{CalculationArg, Number, SassFunction, SassMap, SassNumber, Value},
        Options,
    };

    pub(crate) use std::{
        cmp::Ordering,
        collections::{BTreeMap, BTreeSet},
        sync::Arc,
    };
}
