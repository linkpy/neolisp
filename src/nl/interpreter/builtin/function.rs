use crate::nl::core::object::*;
use crate::nl::interpreter::*;

/// Registers all function builtin forms.
///
pub fn register_builtin_function_forms(scope: &mut Scope) {
    scope.register_special_form("defndynamic", defndynamic);
}

/// `defndynamic` special form.
///
/// `(defndynamic name (arg0 arg1 ...) body...)`
///
pub fn defndynamic(scope: &mut Scope, args: &[Object]) -> Result<Object, Error> {
    if args.len() < 3 {
        return Error::errf(
            &format!(
                "'defndynamic' requires at least 3 arguments, got {} instead.",
                args.len()
            ),
            "defndynamic",
            intern_location!(),
        );
    }

    if !args[0].is_symbol() {
        return Error::errf(
            &format!(
                "'defndynamic' requires a symbol as its first argument, got : {}",
                args[0]
            ),
            "defndynamic",
            intern_location!(),
        );
    }

    let name = args[0].get_symbol().clone();

    if !args[1].is_list() {
        return Error::errf(
            &format!(
                "'defndynamic' requires a list of symbol as its second argument, got : {}",
                args[0]
            ),
            "defndynamic",
            intern_location!(),
        );
    }

    let mut dynform_args = Vec::new();
    let mut dynform_body = Vec::new();

    for (i, arg) in args[1].get_list().iter().enumerate() {
        if !arg.is_symbol() {
            return Error::errf(
                &format!(
                    "'defndynamic' only accepts symbols for the argument list, for the {} argument, got : {}",
                    i+1, arg
                ),
                "defndynamic",
                intern_location!()
            );
        }

        dynform_args.push(arg.get_symbol().clone());
    }

    for obj in args.iter().skip(2) {
        dynform_body.push(obj.clone());
    }

    scope.insert(
        name,
        Binding::DynamicForm(CustomForm {
            location: args[0].get_info().location.clone(),
            arguments: dynform_args,
            body: dynform_body,
        }),
    );

    Ok(Object::nil())
}
