mod checks;
mod flow;
mod function;
mod io;

use checks::*;
use flow::*;
use function::*;
use io::*;

use super::*;
use crate::nl::core::object::*;

/// Registers all builtin forms into the given scope.
///
pub fn register_all_builtin_forms(scope: &mut Scope) {
    scope
        .register_special_form("quote", quote)
        .register_special_form("equote", equote)
        .register_special_form("escape-quote", escape_quote);

    register_builtin_check_forms(scope);
    register_builtin_io_forms(scope);
    register_builtin_flow_forms(scope);
    register_builtin_function_forms(scope);
}

/// `quote` special form.
///
/// `'(a b c)` = `(quote (a b c))`
///
fn quote(_: &mut Scope, args: &[Object]) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::errf(
            &format!(
                "'quote' only receives 1 argument, got {} instead.",
                args.len()
            ),
            "quote",
            intern_location!(),
        );
    }

    Ok(args[0].clone())
}

/// `equote` special form.
///
/// `\`(a b c ,(d e f))` = `(equote (a b c (escape-quote (d e f)))`
///
fn equote(scope: &mut Scope, args: &[Object]) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::errf(
            &format!(
                "'equote' only receives 1 argument, got {} instead.",
                args.len()
            ),
            "equote",
            intern_location!(),
        );
    }

    Error::rethrow(escape_equote(scope, &args[0]), "equote", intern_location!())
}

/// `escape-quote` special form.
///
/// `,(a b c)` = `(escape-quote (a b c))`
///
fn escape_quote(_: &mut Scope, _: &[Object]) -> Result<Object, Error> {
    Error::errf(
        "'escape-quote' must be within a 'equote' form.",
        "escape-quote",
        intern_location!(),
    )
}

/// `escape-quote` special form, only when inside of a `equote` form.
///
/// `\`(a b c ,(d e f))` = `(equote (a b c (escape-quote (d e f)))`
///
fn escape_equote(scope: &mut Scope, object: &Object) -> Result<Object, Error> {
    match object {
        Object::List(_, v) => {
            if v.len() > 0 && v[0].is_symbol() && v[0].get_symbol() == "escape-quote" {
                if v.len() != 2 {
                    return Error::errf(
                        &format!(
                            "'escape-quote' requires 1 argument, got {} instead.",
                            v.len() - 1
                        ),
                        "escape-quote",
                        intern_location!(),
                    );
                }

                return Error::rethrow(evaluate(scope, &v[1]), "escape-quote", intern_location!());
            }

            let mut result = Vec::with_capacity(v.len());

            for x in v {
                result.push(match escape_equote(scope, x) {
                    Ok(v) => v,
                    Err(e) => return e.push_err("escape-quote", intern_location!()),
                })
            }

            Ok(Object::list(result))
        }
        _ => Ok(object.clone()),
    }
}
