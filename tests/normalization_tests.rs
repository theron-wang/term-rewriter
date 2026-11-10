/// The core rewriting logic is tested in rewriting_tests. Here, we just check to see if normalize works as expected.

use term_rewriter::rewriting::NormalizationResult;

use crate::common::{get_arithmetic_trs, get_boolean_trs, get_confluent_nonterminating_trs, parse};

mod common;

#[test]
fn normalize_arithmetic() {
    let trs = get_arithmetic_trs();

    // See rewriting_tests#rewrite_arithmetic_complex
    assert_eq!(
        trs.normalize(parse("+(*(+(0, 0), s(a, b)), +(s(0), s(s(s(0)))))"), 100),
        NormalizationResult::Normalized(parse("s(s(s(+(s(0), 0))))"))
    );
}

#[test]
fn normalize_boolean() {
    let trs = get_boolean_trs();

    // See rewriting_tests#rewrite_boolean_complex
    assert_eq!(
        trs.normalize(
            parse("and(or(not(true), not(or(true, false))), and(not(not(true)), or(true, false)))"),
            100
        ),
        NormalizationResult::Normalized(parse(
            "and(and(or(not(true), not(true)), or(not(true), not(false))), and(true, or(true, false)))"
        ))
    );
}

#[test]
fn already_normalized() {
    let trs = get_boolean_trs();

    assert_eq!(
        trs.normalize(parse("true"), 100),
        NormalizationResult::Normalized(parse("true"))
    );
}

#[test]
fn fail_to_normalize_on_nonterminating_trs() {
    let trs = get_confluent_nonterminating_trs();

    assert_eq!(
        trs.normalize(parse("f(a, a)"), 99),
        NormalizationResult::LimitExceeded(parse("f(a, a)"))
    );
}

#[test]
fn fail_to_normalize_with_limit_0() {
    let trs = get_arithmetic_trs();

    assert_eq!(
        trs.normalize(parse("a"), 0),
        NormalizationResult::LimitExceeded(parse("a"))
    );
}
