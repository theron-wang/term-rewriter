use crate::common::{get_arithmetic_trs, get_boolean_trs, get_confluent_nonterminating_trs, parse};

mod common;

#[test]
fn load_sample_trs_success() {
    get_boolean_trs();
    get_arithmetic_trs();
    get_confluent_nonterminating_trs();
}

#[test]
fn rewrite_arithmetic_simple() {
    let trs = get_arithmetic_trs();

    assert_eq!(trs.rewrite(parse("+(0, a)")), (parse("a"), true));
    assert_eq!(trs.rewrite(parse("*(0, a)")), (parse("0"), true));
    assert_eq!(
        trs.rewrite(parse("+(a, s(b))")),
        (parse("s(+(a, b))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("*(a, s(b))")),
        (parse("+(a, *(a, b))"), true)
    );
}

#[test]
fn rewrite_boolean_simple() {
    let trs = get_boolean_trs();

    assert_eq!(trs.rewrite(parse("not(not(true))")), (parse("true"), true));
    assert_eq!(
        trs.rewrite(parse("not(and(true, false))")),
        (parse("or(not(true), not(false))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("not(or(true, false))")),
        (parse("and(not(true), not(false))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("or(and(true, false), true)")),
        (parse("and(or(true, true), or(false, true))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("or(true, and(true, false))")),
        (parse("and(or(true, true), or(true, false))"), true)
    );
}

#[test]
fn rewrite_arithmetic_no_match() {
    let trs = get_arithmetic_trs();

    assert_eq!(trs.rewrite(parse("+(a, a)")), (parse("+(a, a)"), false));
    assert_eq!(trs.rewrite(parse("1")), (parse("1"), false));
    assert_eq!(
        trs.rewrite(parse("s(+(a, b))")),
        (parse("s(+(a, b))"), false)
    );
    assert_eq!(trs.rewrite(parse("*(?x, ?y)")), (parse("*(?x, ?y)"), false));
}

#[test]
fn rewrite_arithmetic_complex() {
    let trs = get_arithmetic_trs();

    assert_eq!(
        trs.rewrite(parse("+(*(+(0, 0), s(a, b)), +(s(0), s(s(s(0)))))")),
        (parse("+(*(0, s(a, b)), +(s(0), s(s(s(0)))))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("+(*(0, s(a, b)), +(s(0), s(s(s(0)))))")),
        (parse("+(0, +(s(0), s(s(s(0)))))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("+(0, +(s(0), s(s(s(0)))))")),
        (parse("+(s(0), s(s(s(0))))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("+(s(0), s(s(s(0))))")),
        (parse("s(+(s(0), s(s(0))))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("s(+(s(0), s(s(0))))")),
        (parse("s(s(+(s(0), s(0))))"), true)
    );
    assert_eq!(
        trs.rewrite(parse("s(s(+(s(0), s(0))))")),
        (parse("s(s(s(+(s(0), 0))))"), true)
    );
    // Normal form
    assert_eq!(
        trs.rewrite(parse("s(s(s(+(s(0), 0))))")),
        (parse("s(s(s(+(s(0), 0))))"), false)
    );
}

#[test]
fn rewrite_boolean_complex() {
    let trs = get_boolean_trs();

    assert_eq!(
        trs.rewrite(parse(
            "and(or(not(true), not(or(true, false))), and(not(not(true)), or(true, false)))"
        )),
        (
            parse(
                "and(or(not(true), and(not(true), not(false))), and(not(not(true)), or(true, false)))"
            ),
            true
        )
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(or(not(true), and(not(true), not(false))), and(not(not(true)), or(true, false)))"
        )),
        (
            parse(
                "and(and(or(not(true), not(true)), or(not(true), not(false))), and(not(not(true)), or(true, false)))"
            ),
            true
        )
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(not(not(true)), or(true, false)))"
        )),
        (
            parse(
                "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
            ),
            true
        )
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
        )),
        (
            parse(
                "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
            ),
            false
        )
    );
}

#[test]
fn rewrite_nonterminating_confluent() {
    let trs = get_confluent_nonterminating_trs();

    assert_eq!(trs.rewrite(parse("f(b, a)")), (parse("g(b)"), true));
    assert_eq!(trs.rewrite(parse("g(b)")), (parse("r(b)"), true));
    assert_eq!(trs.rewrite(parse("r(b)")), (parse("f(b, a)"), true));
}
