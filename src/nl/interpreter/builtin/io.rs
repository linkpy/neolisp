use crate::nl::core::object::*;
use crate::nl::interpreter::*;

/// Registers all I/O builtin forms.
///
pub fn register_builtin_io_forms(scope: &mut Scope) {
    scope
        .register_eval_form("to-string", to_string)
        .register_eval_form("println", println);
}

/// `to-string` eval form.
///
/// `(to-string v)`
///
fn to_string(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::errf(
            &format!(
                "'to-string' only receives 1 argument, got {} instead.",
                args.len()
            ),
            "to-string",
            intern_location!(),
        );
    }

    Ok(Object::string(format!("{}", args[0])))
}

/// `println` eval form.
///
/// `(println "text")`
///
fn println(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Error::errf(
            &format!(
                "'println' only receives 1 argument, got {} instead.",
                args.len()
            ),
            "to-string",
            intern_location!(),
        );
    }

    if !&args[0].is_string() {
        return Error::errf(
            &format!(
                "'println' only receives a String, got a {} instead.",
                &args[0].kind_string()
            ),
            "to-string",
            intern_location!(),
        );
    }

    println!("{}", &args[0].get_string());
    Ok(Object::nil())
}
