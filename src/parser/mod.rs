mod primitive;
mod value;

use chumsky::{Parser, prelude::Simple};
pub use value::value;

use crate::{Error, Result};

#[must_use]
pub fn collect_errors(errors: Vec<Simple<char>>) -> Error {
    let error_strings: Vec<String> = errors.into_iter().map(|err| err.to_string()).collect();
    Error::msg(error_strings.join("\n"))
}

pub fn parse<T, S: AsRef<str>>(
    s: S,
    parser: impl Parser<char, T, Error = Simple<char>>,
) -> Result<T> {
    parser.parse(s.as_ref()).map_err(collect_errors)
}

#[cfg(test)]
mod tests {
    use crate::{Result, Value, parser};

    fn parse_value<S: AsRef<str>>(s: S) -> Result<Value> {
        parser::parse(s, parser::value())
    }

    #[test]
    fn symbol() {
        assert_eq!(Value::symbol("nil"), parse_value("nil").unwrap());
        assert_eq!(Value::symbol("cons"), parse_value("cons").unwrap());
        assert_eq!(Value::symbol("true"), parse_value("true").unwrap());
        assert_eq!(Value::symbol("false"), parse_value("false").unwrap());
        assert_eq!(Value::symbol("boolean"), parse_value("boolean").unwrap());
        assert_eq!(Value::symbol("symbol"), parse_value("symbol").unwrap());
        assert_eq!(Value::symbol("number"), parse_value("number").unwrap());
        assert_eq!(Value::symbol("string"), parse_value("string").unwrap());
        assert_eq!(Value::symbol("quote"), parse_value("quote").unwrap());
        assert_eq!(
            Value::symbol("quasiquote"),
            parse_value("quasiquote").unwrap()
        );
        assert_eq!(Value::symbol("unquote"), parse_value("unquote").unwrap());
        assert_eq!(
            Value::symbol("unquote-splicing"),
            parse_value("unquote-splicing").unwrap()
        );
        assert_eq!(Value::symbol("fn"), parse_value("fn").unwrap());
        assert_eq!(
            Value::symbol("native-fn"),
            parse_value("native-fn").unwrap()
        );
    }

    #[test]
    fn number() {
        assert_eq!(Value::from(0.0), parse_value("0").unwrap());
        assert_eq!(Value::from(1.0), parse_value("1.0").unwrap());
        assert_eq!(Value::from(1.618_034), parse_value("1.618034").unwrap());
        assert_eq!(Value::from(-2e3), parse_value("-2e3").unwrap());
        assert_eq!(Value::from(4E5), parse_value("4E5").unwrap());
        assert_eq!(Value::from(6e-7), parse_value("6e-7").unwrap());
        assert_eq!(Value::from(8e+9), parse_value("8e+9").unwrap());

        assert!(parse_value(".").is_err());
        assert!(parse_value(".0").is_err());
        assert!(parse_value("01").is_err());
    }

    #[test]
    #[allow(clippy::manual_string_new)]
    fn string() {
        assert_eq!(Value::from("".to_string()), parse_value(r#""""#).unwrap());
        assert_eq!(
            Value::from("abc".to_string()),
            parse_value(r#""abc""#).unwrap()
        );
        assert_eq!(
            Value::from("\\".to_string()),
            parse_value(r#""\\""#).unwrap()
        );
        assert_eq!(
            Value::from("\0".to_string()),
            parse_value(r#""\0""#).unwrap()
        );
        assert_eq!(
            Value::from("\n".to_string()),
            parse_value(r#""\n""#).unwrap()
        );
        assert_eq!(
            Value::from("\r".to_string()),
            parse_value(r#""\r""#).unwrap()
        );
        assert_eq!(
            Value::from("\t".to_string()),
            parse_value(r#""\t""#).unwrap()
        );
        assert_eq!(
            Value::from("\u{0}".to_string()),
            parse_value(r#""\u{0}""#).unwrap()
        );
        assert_eq!(
            Value::from("\u{211D}".to_string()),
            parse_value(r#""\u{211D}""#).unwrap()
        );
        assert_eq!(
            Value::from("\u{10FFFF}".to_string()),
            parse_value(r#""\u{10FFFF}""#).unwrap()
        );

        assert!(parse_value(r#"""#).is_err());
        assert!(parse_value(r#""\""#).is_err());
        assert!(parse_value(r#"""""#).is_err());
        assert!(parse_value(r#""\u211D""#).is_err());
        assert!(parse_value(r#""\u{01234567}""#).is_err());
    }

    #[test]
    fn quotes() {
        assert_eq!(Value::quote(Value::symbol("x")), parse_value("'x").unwrap());
        assert_eq!(
            Value::quasiquote(Value::symbol("x")),
            parse_value("`x").unwrap()
        );
        assert_eq!(
            Value::unquote(Value::symbol("x")),
            parse_value(",x").unwrap()
        );
        assert_eq!(
            Value::unquote_splicing(Value::symbol("x")),
            parse_value(",@x").unwrap()
        );
    }

    #[test]
    fn lists() {
        assert_eq!(Value::nil(), parse_value("()").unwrap());
        assert_eq!(Value::nil(), parse_value("[]").unwrap());
        assert_eq!(Value::list([Value::nil()]), parse_value("(())").unwrap());

        assert!(parse_value("(").is_err());
        assert!(parse_value(")").is_err());
        assert!(parse_value("())").is_err());
        assert!(parse_value("(()").is_err());
        assert!(parse_value("([)]").is_err());
    }
}
