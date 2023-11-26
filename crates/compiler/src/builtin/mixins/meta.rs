use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::builtin::builtin_imports::*;
use crate::ast::{Configuration, ConfiguredValue};
use crate::error::SassResult;
use crate::Visitor;

pub(crate) fn load_css(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<()> {
    args.max_args(2)?;

    let span = args.span();

    let url = args
        .get_err(0, "module")?
        .assert_string_with_name("module", args.span())?
        .0;

    let with = match args.default_arg(1, "with", Value::Null) {
        Value::Map(map) => Some(map),
        Value::List(v, ..) if v.is_empty() => Some(SassMap::new()),
        Value::ArgList(v) if v.is_empty() => Some(SassMap::new()),
        Value::Null => None,
        v => return Err((format!("$with: {} is not a map.", v.inspect(span)?), span).into()),
    };

    let mut configuration = Configuration::empty();

    if let Some(with) = with {
        visitor.emit_warning("`grass` does not currently support the $with parameter of load-css. This file will be imported the same way it would using `@import`.", args.span());

        let mut values = BTreeMap::new();
        for (key, value) in with {
            let name =
                Identifier::from(key.node.assert_string_with_name("with key", args.span())?.0);

            if values.contains_key(&name) {
                // todo: write test for this
                return Err((
                    format!("The variable {name} was configured twice.", name = name),
                    key.span,
                )
                    .into());
            }

            values.insert(name, ConfiguredValue::explicit(value, args.span()));
        }

        configuration = Configuration::explicit(values, args.span());
    }

    let _configuration = Arc::new(RefCell::new(configuration));

    let style_sheet = visitor.load_style_sheet(url.as_ref(), false, args.span())?;

    visitor.visit_stylesheet(style_sheet)?;

    // todo: support the $with argument to load-css
    // visitor.load_module(
    //     url.as_ref(),
    //     Some(Arc::clone(&configuration)),
    //     true,
    //     args.span(),
    //     |visitor, module, stylesheet| {
    //         // (*module).borrow()
    //         Ok(())
    //     },
    // )?;

    // Visitor::assert_configuration_is_empty(&configuration, true)?;

    Ok(())
}

pub(crate) fn apply(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<()> {
    args.min_args(1)?;
    let span = args.span();

    let mixin = match args.get_err(0, "mixin")? {
        Value::MixinRef(m) => *m,
        v => {
            return Err((
                format!("$mixin: {} is not a mixin reference.", v.inspect(span)?),
                span
            ).into());
        }
    };

    args.remove_positional(0);

    visitor.run_mixin_callable_with_maybe_evaled(
        mixin,
        visitor.env.content.clone(),
        MaybeEvaledArguments::Evaled(args),
        span
    )
}

pub(crate) fn declare(f: &mut GlobalMixinMap) {
    f.insert("load-css", BuiltinMixin::new(load_css));
    f.insert("apply", BuiltinMixin::new(apply));
}