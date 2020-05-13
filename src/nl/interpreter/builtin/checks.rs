use crate::nl::core::object::*;
use crate::nl::interpreter::*;

/// Registers all builtin checks forms into the given scope.
///
pub fn register_builtin_check_forms(scope: &mut Scope) {
    scope
        .register_eval_form("is-nil?", is_nil)
        .register_eval_form("is-bool?", is_bool)
        .register_eval_form("is-integer?", is_integer)
        .register_eval_form("is-float?", is_float)
        .register_eval_form("is-char?", is_char)
        .register_eval_form("is-string?", is_string)
        .register_eval_form("is-keyword?", is_keyword)
        .register_eval_form("is-symbol?", is_symbol)
        .register_eval_form("is-list?", is_list);
}

/// `is-nil?` eval form.
///
/// `(is-nil? v)`
///
fn is_nil(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-nil?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_nil()))
}

/// `is-bool?` eval form.
///
/// `(is-bool? v)`
///
fn is_bool(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-bool?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_bool()))
}

/// `is-integer?` eval form.
///
/// `(is-integer? v)`
///
fn is_integer(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-int?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_integer()))
}

/// `is-float?` eval form.
///
/// `(is-float? v)`
///
fn is_float(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-float?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_float()))
}

/// `is-char?` eval form.
///
/// `(is-char? v)`
///
fn is_char(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-char?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_char()))
}

/// `is-string?` eval form.
///
/// `(is-string? v)`
///
fn is_string(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-string?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_string()))
}

/// `is-keyword?` eval form.
///
/// `(is-keyword? v)`
///
fn is_keyword(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-keyword?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_keyword()))
}

/// `is-symbol?` eval form.
///
/// `(is-symbol? v)`
///
fn is_symbol(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-symbol?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_symbol()))
}

/// `is-list?` eval form.
///
/// `(is-list? v)`
///
fn is_list(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::err(&format!(
            "'is-list?' only receives 1 argument, got {} instead.",
            args.len()
        ));
    }

    Ok(Object::bool(args[0].is_list()))
}
