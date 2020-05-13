use crate::nl::core::object::Object;

use super::binding::*;
use super::scope::*;

pub use crate::nl::core::error::*;

pub fn evaluate(scope: &mut Scope, object: &Object) -> Result<Object, Error> {
    let r = match object {
        Object::Symbol(_, _) => evaluate_symbol(scope, object),
        Object::List(_, expr) => evaluate_expression(scope, expr),
        _ => Ok(object.clone()),
    };

    r
}

fn evaluate_symbol(scope: &mut Scope, object: &Object) -> Result<Object, Error> {
    let symbol = object.get_symbol();

    if !scope.has_binding(symbol) {
        return Error::errf(
            &format!("unbound symbol '{}'", symbol),
            symbol,
            object.get_info().location.clone(),
        );
    }

    let binding = scope.get_binding(symbol).unwrap();

    match binding {
        Binding::DynamicVariable(object) => Ok(object.clone()),
        _ => Error::errf(
            &format!("'{}' is not a variable.", symbol),
            symbol,
            object.get_info().location.clone(),
        ),
    }
}

fn evaluate_expression(scope: &mut Scope, expr: &Vec<Object>) -> Result<Object, Error> {
    if expr.len() == 0 {
        return Ok(Object::nil());
    }

    let operator = &expr[0];
    let rest = &expr[1..];

    match operator {
        Object::Symbol(info, name) => {
            if !scope.has_binding(name) {
                return Error::errf(
                    &format!("unbound symbol '{}'", name),
                    name,
                    info.location.clone(),
                );
            }

            let binding = scope.get_binding(name).unwrap();

            let r = match binding {
                Binding::DynamicVariable(_) => Error::errf(
                    &format!("'{}' : expected an operator, got a variable.", name),
                    name,
                    info.location.clone(),
                ),
                Binding::SpecialForm(func) => func(scope, rest),
                Binding::EvalForm(func) => func(match evaluate_list(scope, rest) {
                    Ok(v) => v,
                    Err(v) => return v.push_err(name, info.location.clone()),
                }),
                Binding::DynamicForm(form) => {
                    let f = form.clone();
                    evaluate_dynamic_form(scope, name, rest, f)
                }
                Binding::MacroForm(form) => {
                    let f = form.clone();
                    evaluate_macro_form(scope, name, rest, f)
                }
            };

            Error::rethrow(r, name, info.location.clone())
        }
        _ => Error::errf(
            &format!("invalid s-expression : {}", operator),
            "neolisp",
            intern_location!(),
        ),
    }
}

fn evaluate_list(scope: &mut Scope, list: &[Object]) -> Result<Vec<Object>, Error> {
    let mut result = Vec::with_capacity(list.len());

    for x in list {
        match evaluate(scope, x) {
            Ok(v) => result.push(v),
            Err(v) => return Err(v),
        }
    }

    Ok(result)
}

fn evaluate_dynamic_form(
    scope: &mut Scope,
    name: &String,
    args: &[Object],
    form: CustomForm,
) -> Result<Object, Error> {
    if args.len() != form.arguments.len() {
        return Error::errf(
            &format!(
                "'{}' requires {} arguments, got {} instead.",
                name,
                form.arguments.len(),
                args.len()
            ),
            name,
            form.location,
        );
    }

    scope.enter_loop_boundary(Mode::Evaluation);

    for i in 0..args.len() {
        let arg_name = &form.arguments[i];
        let expr = match evaluate(scope, &args[i]) {
            Ok(v) => v,
            Err(v) => return v.push_err(name, form.location),
        };

        scope.insert(arg_name.clone(), Binding::DynamicVariable(expr));
    }

    let mut result = Object::nil();

    for expr in &form.body {
        match evaluate(scope, expr) {
            Ok(v) => result = v,
            Err(v) => return v.push_err(name, form.location),
        }
    }

    scope.leave();
    Ok(result)
}

fn evaluate_macro_form(
    scope: &mut Scope,
    name: &String,
    args: &[Object],
    form: CustomForm,
) -> Result<Object, Error> {
    if args.len() != form.arguments.len() {
        return Error::errf(
            &format!(
                "'{}' requires {} arguments, got {} instead.",
                name,
                form.arguments.len(),
                args.len()
            ),
            name,
            form.location,
        );
    }

    scope.enter_loop_boundary(Mode::Macro);

    for i in 0..args.len() {
        let arg_name = &form.arguments[i];
        scope.insert(arg_name.clone(), Binding::DynamicVariable(args[i].clone()));
    }

    let mut result = Object::nil();

    for expr in &form.body {
        match evaluate(scope, expr) {
            Ok(v) => result = v,
            Err(v) => return v.push_err(name, form.location),
        }
    }

    scope.leave();
    Ok(result)
}
