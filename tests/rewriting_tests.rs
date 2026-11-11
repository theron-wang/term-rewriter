use term_rewriter::rewriting::RewriteResult;

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

    assert_eq!(
        trs.rewrite(parse("+(0, a)")),
        RewriteResult::Rewritten(parse("a"))
    );
    assert_eq!(
        trs.rewrite(parse("*(0, a)")),
        RewriteResult::Rewritten(parse("0"))
    );
    assert_eq!(
        trs.rewrite(parse("+(a, s(b))")),
        RewriteResult::Rewritten(parse("s(+(a, b))"))
    );
    assert_eq!(
        trs.rewrite(parse("*(a, s(b))")),
        RewriteResult::Rewritten(parse("+(a, *(a, b))"))
    );
}

#[test]
fn rewrite_boolean_simple() {
    let trs = get_boolean_trs();

    assert_eq!(
        trs.rewrite(parse("not(not(true))")),
        RewriteResult::Rewritten(parse("true"))
    );
    assert_eq!(
        trs.rewrite(parse("not(and(true, false))")),
        RewriteResult::Rewritten(parse("or(not(true), not(false))"))
    );
    assert_eq!(
        trs.rewrite(parse("not(or(true, false))")),
        RewriteResult::Rewritten(parse("and(not(true), not(false))"))
    );
    assert_eq!(
        trs.rewrite(parse("or(and(true, false), true)")),
        RewriteResult::Rewritten(parse("and(or(true, true), or(false, true))"))
    );
    assert_eq!(
        trs.rewrite(parse("or(true, and(true, false))")),
        RewriteResult::Rewritten(parse("and(or(true, true), or(true, false))"))
    );
}

#[test]
fn rewrite_arithmetic_no_match() {
    let trs = get_arithmetic_trs();

    assert_eq!(
        trs.rewrite(parse("+(a, a)")),
        RewriteResult::Unchanged(parse("+(a, a)"))
    );
    assert_eq!(
        trs.rewrite(parse("1")),
        RewriteResult::Unchanged(parse("1"))
    );
    assert_eq!(
        trs.rewrite(parse("s(+(a, b))")),
        RewriteResult::Unchanged(parse("s(+(a, b))"))
    );
    assert_eq!(
        trs.rewrite(parse("*(?x, ?y)")),
        RewriteResult::Unchanged(parse("*(?x, ?y)"))
    );
}

#[test]
fn rewrite_arithmetic_complex() {
    let trs = get_arithmetic_trs();

    assert_eq!(
        trs.rewrite(parse("+(*(+(0, 0), s(a, b)), +(s(0), s(s(s(0)))))")),
        RewriteResult::Rewritten(parse("+(*(0, s(a, b)), +(s(0), s(s(s(0)))))"))
    );
    assert_eq!(
        trs.rewrite(parse("+(*(0, s(a, b)), +(s(0), s(s(s(0)))))")),
        RewriteResult::Rewritten(parse("+(0, +(s(0), s(s(s(0)))))"))
    );
    assert_eq!(
        trs.rewrite(parse("+(0, +(s(0), s(s(s(0)))))")),
        RewriteResult::Rewritten(parse("+(s(0), s(s(s(0))))"))
    );
    assert_eq!(
        trs.rewrite(parse("+(s(0), s(s(s(0))))")),
        RewriteResult::Rewritten(parse("s(+(s(0), s(s(0))))"))
    );
    assert_eq!(
        trs.rewrite(parse("s(+(s(0), s(s(0))))")),
        RewriteResult::Rewritten(parse("s(s(+(s(0), s(0))))"))
    );
    assert_eq!(
        trs.rewrite(parse("s(s(+(s(0), s(0))))")),
        RewriteResult::Rewritten(parse("s(s(s(+(s(0), 0))))"))
    );
    // Normal form
    assert_eq!(
        trs.rewrite(parse("s(s(s(+(s(0), 0))))")),
        RewriteResult::Unchanged(parse("s(s(s(+(s(0), 0))))"))
    );
}

#[test]
fn rewrite_boolean_complex() {
    let trs = get_boolean_trs();

    assert_eq!(
        trs.rewrite(parse(
            "and(or(not(true), not(or(true, false))), and(not(not(true)), or(true, false)))"
        )),
        RewriteResult::Rewritten(parse(
            "and(or(not(true), and(not(true), not(false))), and(not(not(true)), or(true, false)))"
        ))
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(or(not(true), and(not(true), not(false))), and(not(not(true)), or(true, false)))"
        )),
        RewriteResult::Rewritten(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(not(not(true)), or(true, false)))"
        ))
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(not(not(true)), or(true, false)))"
        )),
        RewriteResult::Rewritten(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
        ))
    );

    assert_eq!(
        trs.rewrite(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
        )),
        RewriteResult::Unchanged(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
        ))
    );
}

#[test]
fn rewrite_nonterminating_confluent() {
    let trs = get_confluent_nonterminating_trs();

    assert_eq!(
        trs.rewrite(parse("f(b, a)")),
        RewriteResult::Rewritten(parse("g(b)"))
    );
    assert_eq!(
        trs.rewrite(parse("g(b)")),
        RewriteResult::Rewritten(parse("r(b)"))
    );
    assert_eq!(
        trs.rewrite(parse("r(b)")),
        RewriteResult::Rewritten(parse("f(b, a)"))
    );
}
