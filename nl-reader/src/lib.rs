pub use nl_om::Object;

/// Extra information given by the reader.
///
pub struct Location(usize, usize);

peg::parser! { pub grammar parser() for str {

    pub rule exprs() -> Vec<Object<Location>>
        = expr()+

    pub rule expr() -> Object<Location>
        = quote_macro()
        / equote_macro()
        / escape_macro()
        / list()
        / float()
        / integer()
        / character()
        / string()
        / raw_string()
        / nil()
        / boolean()
        / keyword()
        / symbol()


    rule quote_macro() -> Object<Location>
        = _ from:position!() "'" e:expr() to:position!() _
            { make_quote_macro(from, to, e) }
    rule equote_macro() -> Object<Location>
        = _ from:position!() "`" e:expr() to:position!() _
            { make_equote_macro(from, to, e) }
    rule escape_macro() -> Object<Location>
        = _ from:position!() "," e:expr() to:position!() _
            { make_escape_macro(from, to, e) }
    rule list() -> Object<Location>
        = _ from:position!() "(" _ l:(expr()*) _ ")" to:position!() _
            { make_list(from, to, l) }
    rule integer() -> Object<Location>
        = _ from:position!() "0x" v:$(hexadecimal_digit()+) to:position!() _
            { make_hexa(from, to, v) }
        / _ from:position!() "0o" v:$(octal_digit()+) to:position!() _
            { make_octal(from, to, v) }
        / _ from:position!() "0b" v:$(binary_digit()+) to:position!() _
            { make_bin(from, to, v) }
        / _ from:position!() v:$(decimal_digit()+) to:position!() _
            { make_dec(from, to, v) }
    rule float() -> Object<Location>
        = _ from:position!() v:$(decimal_digit()+ "." decimal_digit()+) to:position!() _
            { make_float(from, to, v) }
    rule character() -> Object<Location>
        = _ from:position!() "#" c:char_name() to:position!() _
            { make_char(from, to, c) }
        / _ from:position!() "#" c:escaped_char() to:position!() _
            { make_char(from, to, c) }
    rule string() -> Object<Location>
        = _ from:position!() "\"" v:(string_char()*) "\"" to:position!() _
            { make_string(from, to, v) }
    rule raw_string() -> Object<Location>
        = _ from:position!() "r\"" v:(raw_string_char()*) "\"" to:position!() _
            { make_string(from, to, v) }
    rule nil() -> Object<Location>
        = _ from:position!() "nil" to:position!() _
            { make_nil(from, to) }
    rule boolean() -> Object<Location>
        = _ from:position!() "true" to:position!() _
            { make_bool(from, to, true) }
        / _ from:position!() "false" to:position!() _
            { make_bool(from, to, false) }
    rule keyword() -> Object<Location>
        = _ from:position!() ":" v:$(name()) to:position!() _
            { make_keyword(from, to, v) }
    rule symbol() -> Object<Location>
        = _ from:position!() v:$(name()) to:position!() _
            { make_symbol(from, to, v) }


    rule hexadecimal_digit()
        = decimal_digit() / ['a'..='f'] / ['A'..='F']
    rule octal_digit()
        = ['0'..='7'] / "_"
    rule binary_digit()
        = "0" / "1" / "_"
    rule decimal_digit()
        = ['0'..='9'] / "_"

    rule escaped_char() -> char
        = "\\n"                                         { '\n' }
        / "\\t"                                         { '\t' }
        / "\\r"                                         { '\r' }
        / "\\\""                                        { '\"' }
        / !"\\" c:$([_])                                { c.chars().next().unwrap() }
    rule char_name() -> char
        = "space"                                       { ' ' }
        / "newline"                                     { '\n' }
        / "tab"                                         { '\t' }
        / "carriage-return"                             { '\r' }
    rule string_char() -> char
        = !"\"" c:escaped_char()                        { c }
    rule raw_string_char() -> char
        = "\"\""                                        { '"' }
        / !"\"" c:$([_])                                { c.chars().next().unwrap() }

    rule name()
        = name_char()+
    rule name_char()
        = ['a'..='z']
        / ['A'..='Z']
        / ['0'..='9']
        / "+" / "-" / "*" / "/" / "%" / "^" / "~"
        / ">" / "<" / "="
        / "?" / "." / ":" / "!"
        / "_"


    rule _()
        = (spacing() / comment())*

    rule spacing()
        = " " / "\t" / "\r" / "\n"
    rule comment()
        = ";" (!eol() [_])* eol()

    rule eol()
        = "\r\n"
        / "\n\r"
        / "\r"
        / "\n"
    rule eof()
        = ![_]

}}

fn make_quote_macro(f: usize, t: usize, e: Object<Location>) -> Object<Location> {
    Object::List(
        Location(f, t),
        vec![Object::Symbol(Location(f, f + 1), "quote".to_string()), e],
    )
}

fn make_equote_macro(f: usize, t: usize, e: Object<Location>) -> Object<Location> {
    Object::List(
        Location(f, t),
        vec![Object::Symbol(Location(f, f + 1), "equote".to_string()), e],
    )
}

fn make_escape_macro(f: usize, t: usize, e: Object<Location>) -> Object<Location> {
    Object::List(
        Location(f, t),
        vec![
            Object::Symbol(Location(f, f + 1), "escape-quote".to_string()),
            e,
        ],
    )
}

fn make_list(f: usize, t: usize, l: Vec<Object<Location>>) -> Object<Location> {
    Object::List(Location(f, t), l)
}

fn make_hexa(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Integer(
        Location(f, t),
        i32::from_str_radix(&v.replace("_", ""), 16).unwrap(),
    )
}

fn make_octal(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Integer(
        Location(f, t),
        i32::from_str_radix(&v.replace("_", ""), 8).unwrap(),
    )
}

fn make_bin(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Integer(
        Location(f, t),
        i32::from_str_radix(&v.replace("_", ""), 2).unwrap(),
    )
}

fn make_dec(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Integer(
        Location(f, t),
        i32::from_str_radix(&v.replace("_", ""), 10).unwrap(),
    )
}

fn make_float(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Float(Location(f, t), v.replace("_", "").parse::<f32>().unwrap())
}

fn make_char(f: usize, t: usize, v: char) -> Object<Location> {
    Object::Char(Location(f, t), v)
}

fn make_string(f: usize, t: usize, v: Vec<char>) -> Object<Location> {
    Object::String(Location(f, t), v.iter().collect())
}

fn make_nil(f: usize, t: usize) -> Object<Location> {
    Object::Nil(Location(f, t))
}

fn make_bool(f: usize, t: usize, v: bool) -> Object<Location> {
    Object::Bool(Location(f, t), v)
}

fn make_keyword(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Keyword(Location(f, t), v.to_string())
}

fn make_symbol(f: usize, t: usize, v: &str) -> Object<Location> {
    Object::Symbol(Location(f, t), v.to_string())
}
