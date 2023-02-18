use chumsky::prelude::*;

fn parse_float<S: AsRef<str>>(s: S) -> f64 {
    let str = s.as_ref();
    str.parse()
        .unwrap_or_else(|_| panic!("invalid number: {str}"))
}

pub fn float() -> impl Parser<char, f64, Error = Simple<char>> {
    let frac = just('.').chain(text::digits(10));

    let exp = just('e')
        .or(just('E'))
        .chain(just('+').or(just('-')).or_not())
        .chain(text::digits(10));

    just('-')
        .or_not()
        .chain(text::int(10))
        .chain(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .collect::<String>()
        .map(parse_float)
}

pub fn string() -> impl Parser<char, String, Error = Simple<char>> {
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('"'))
            .or(just('b').to('\x08'))
            .or(just('f').to('\x0C'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('u').ignore_then(
                filter(char::is_ascii_hexdigit)
                    .repeated()
                    .exactly(4)
                    .collect::<String>()
                    .validate(|digits, span, emit| {
                        char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emit(Simple::custom(
                                    span,
                                    format!("invalid Unicode character: \\u{digits}"),
                                ));
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                    }),
            )),
    );

    let string_body = filter(|&c| c != '\\' && c != '"').or(escape).repeated();
    string_body.delimited_by(just('"'), just('"')).collect()
}
