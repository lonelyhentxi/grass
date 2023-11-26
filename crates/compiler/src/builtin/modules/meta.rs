use crate::builtin::builtin_imports::*;

use crate::builtin::{
    meta::{
        call, content_exists, feature_exists, function_exists, get_function, get_mixin,
        global_variable_exists, inspect, keywords, mixin_exists, type_of, variable_exists,
    },
    mixins::meta::load_css,
    modules::Module,
};
use crate::builtin::mixins::meta::apply;
use crate::serializer::serialize_calculation_arg;

fn module_functions(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;

    let module = Identifier::from(
        args.get_err(0, "module")?
            .assert_string_with_name("module", args.span())?
            .0,
    );

    Ok(Value::Map(
        (*(*visitor.env.modules).borrow().get(module, args.span())?)
            .borrow()
            .functions(args.span()),
    ))
}

fn module_variables(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;

    let module = Identifier::from(
        args.get_err(0, "module")?
            .assert_string_with_name("module", args.span())?
            .0,
    );

    Ok(Value::Map(
        (*(*visitor.env.modules).borrow().get(module, args.span())?)
            .borrow()
            .variables(args.span()),
    ))
}

fn calc_args(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;

    let calc = match args.get_err(0, "calc")? {
        Value::Calculation(calc) => calc,
        v => {
            return Err((
                format!("$calc: {} is not a calculation.", v.inspect(args.span())?),
                args.span(),
            )
                .into())
        }
    };

    let args = calc
        .args
        .into_iter()
        .map(|arg| {
            Ok(match arg {
                CalculationArg::Number(num) => Value::Dimension(num),
                CalculationArg::Calculation(calc) => Value::Calculation(calc),
                CalculationArg::String(s) | CalculationArg::Interpolation(s) => {
                    Value::String(s, QuoteKind::None)
                }
                CalculationArg::Operation { .. } => Value::String(
                    serialize_calculation_arg(&arg, visitor.options, args.span())?,
                    QuoteKind::None,
                ),
            })
        })
        .collect::<SassResult<Vec<_>>>()?;

    Ok(Value::List(args, ListSeparator::Comma, Brackets::None))
}

fn calc_name(mut args: ArgumentResult, _visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;

    let calc = match args.get_err(0, "calc")? {
        Value::Calculation(calc) => calc,
        v => {
            return Err((
                format!("$calc: {} is not a calculation.", v.inspect(args.span())?),
                args.span(),
            )
                .into())
        }
    };

    Ok(Value::String(calc.name.to_string(), QuoteKind::Quoted))
}

pub(crate) fn declare(f: &mut Module) {
    f.insert_builtin("feature-exists", feature_exists);
    f.insert_builtin("inspect", inspect);
    f.insert_builtin("type-of", type_of);
    f.insert_builtin("keywords", keywords);
    f.insert_builtin("global-variable-exists", global_variable_exists);
    f.insert_builtin("variable-exists", variable_exists);
    f.insert_builtin("function-exists", function_exists);
    f.insert_builtin("mixin-exists", mixin_exists);
    f.insert_builtin("content-exists", content_exists);
    f.insert_builtin("module-variables", module_variables);
    f.insert_builtin("module-functions", module_functions);
    f.insert_builtin("get-function", get_function);
    f.insert_builtin("get-mixin", get_mixin);
    f.insert_builtin("call", call);
    f.insert_builtin("calc-args", calc_args);
    f.insert_builtin("calc-name", calc_name);

    f.insert_builtin_mixin("load-css", load_css);
    f.insert_builtin_mixin("apply", apply);
}
