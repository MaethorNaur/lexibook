use nom::types::CompleteStr;
use nom::{alpha, not_line_ending, space};
use std::str;

use super::ast::*;

fn vec_join(vec: Vec<&str>) -> &str {
    Box::leak(vec.join("").into_boxed_str())
}
fn to_string(input: CompleteStr) -> &str {
    input.0
}

named!(semi_column<CompleteStr,()>, do_parse!(
    opt!(space)>>
    char!(':')>>
    opt!(space)>>
    ()
));

named!(syllable<CompleteStr,Syllable>,
    alt!(
        value!(Syllable::Optional,char!('?'))|
        map_opt!(take!(1), |s: CompleteStr|
            s.0.chars().next().filter(|c|c.is_uppercase()).map(Syllable::Class) )|
        map!(alpha, |s| Syllable::Sound(to_string(s)))
    )
);
named!(words_expr<CompleteStr, Expr>,
    do_parse!(
        tag!("words") >>
        semi_column >>
        syllables: ws!(many1!(many1!(syllable))) >>
        (Expr::Words(syllables))
    )
);

named!(letter<CompleteStr,(&str, &str)>,
   do_parse!(
        opt!(space)>>
        letter: alpha >>
        semi_column >>
        sounds: alpha >>
        opt!(space)>>
        ((letter.0, sounds.0))
    )
);
named!(letter_expr<CompleteStr,Expr>,
    do_parse!(
        tag!("letters") >>
        semi_column >>
        letters: ws!(separated_list!(char!(','), letter)) >>
        (Expr::Letters(letters))
    )
);

named!(class_expr<CompleteStr,Expr>,
    do_parse!(
        class: map!(
            many1!(map!(alt!(alpha|tag!("_")),to_string)),
            vec_join
         ) >>
        opt!(space) >>
        char!('=') >>
        opt!(space) >>
        letters: separated_list!(char!(' '), map!(alpha, |s| s.0)) >>
        (Expr::Class(&class,letters))
    )
);
named!(comments<CompleteStr,Expr>,
    do_parse!(
        char!('%') >>
        many0!(not_line_ending) >>
        (Expr::Comments)
    )
);
named!(expr_choice<CompleteStr,Expr>,
    alt!(
        comments |
        letter_expr|
        words_expr |
        class_expr
    )
);
named!(pub do_parse<CompleteStr,Vec<Expr>>,
    do_parse!(
        exprs: ws!(many0!(expr_choice)) >>
        eof!() >>
        (exprs)
    )
);
