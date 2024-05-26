use chumsky::prelude::*;

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
        .validate(|num_str, span, emit| {
            num_str.parse().unwrap_or_else(|_| {
                emit(Simple::custom(span, format!("invalid number: {num_str}")));
                0.0
            })
        })
}

pub fn string() -> impl Parser<char, String, Error = Simple<char>> {
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('"'))
            .or(just('0').to('\0'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('u').ignore_then(
                filter(char::is_ascii_hexdigit)
                    .repeated()
                    .at_least(1)
                    .at_most(6)
                    .delimited_by(just('{'), just('}'))
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
