use std::collections::HashSet;

use term_rewriter::{rewriting::{RewriteRule, TermRewritingSystem}, terms::Term};

use crate::common::{get_arithmetic_trs, get_boolean_trs, get_confluent_nonterminating_trs, parse};

#[allow(dead_code)]
mod common;

/// Utility method to convert critical pair vectors into vectors of string tuples for easy comparison.
fn to_hashset(tuples: Vec<(Term, Term)>) -> HashSet<(Term, Term)> {
    tuples.into_iter().collect()
}

/// Pretty prints the left and right critical pair vectors.
fn pretty_print(left: &Vec<(Term, Term)>, right: &Vec<(Term, Term)>) {
    println!(
        "left: {:#?}",
        left
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<(String, String)>>()
    );
    println!(
        "right: {:#?}",
        right
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<(String, String)>>()
    );
}

#[test]
fn calculate_all_arithmetic_trs_critical_pairs() {
    let trs = get_arithmetic_trs();

    let calculated = trs.critical_pairs();
    let expected = vec![
        (parse("s(?y)"), parse("s(+(0, ?y))")),
        (parse("0"), parse("+(0, *(0, ?y))")),
        // At the root, a critical pair (a, b) means (b, a) is also a critical pair.
        (parse("s(+(0, ?y))"), parse("s(?y)")),
        (parse("+(0, *(0, ?y))"), parse("0")),
    ];

    // Pretty print since assert_eq is very hard to read (stdout is only shown on failures)
    pretty_print(&calculated, &expected);

    assert!(to_hashset(calculated) == to_hashset(expected));
}

#[test]
fn calculate_all_boolean_trs_critical_pairs() {
    let trs = get_boolean_trs();

    let calculated = trs.critical_pairs();
    let expected = vec![
        // not(not(and(?x, ?y)))
        (parse("and(?x, ?y)"), parse("not(or(not(?x), not(?y)))")),
        // not(not(or(?x, ?y)))
        (parse("or(?x, ?y)"), parse("not(and(not(?x), not(?y)))")),
        // not(or(and(?x, ?y), ?z)
        (
            parse("and(not(and(?x, ?y)), not(?z))"),
            parse("not(and(or(?x, ?z), or(?y, ?z)))"),
        ),
        // not(or(?x, and(?y, ?z)))
        (
            parse("and(not(?x), not(and(?y, ?z)))"),
            parse("not(and(or(?x, ?y), or(?x, ?z)))"),
        ),
        // or(and(?x, ?y), and(?y, ?z))
        (
            parse("and(or(?x, and(?y, ?z)), or(?y, and(?y, ?z)))"),
            parse("and(or(and(?x, ?y), ?y), or(and(?x, ?y), ?z))"),
        ),
        // At the root, a critical pair (a, b) means (b, a) is also a critical pair.
        // 5:
        (
            parse("and(or(and(?x, ?y), ?y), or(and(?x, ?y), ?z))"),
            parse("and(or(?x, and(?y, ?z)), or(?y, and(?y, ?z)))"),
        ),
    ];

    // Pretty print since assert_eq is very hard to read (stdout is only shown on failures)
    pretty_print(&calculated, &expected);

    assert!(to_hashset(calculated) == to_hashset(expected));
}

#[test]
fn calculate_all_confluent_nonterminating_trs_critical_pairs() {
    let trs = get_confluent_nonterminating_trs();

    let calculated = trs.critical_pairs();
    let expected = vec![
        (parse("g(?x)"), parse("h(?x)")),
        // At the root, a critical pair (a, b) means (b, a) is also a critical pair.
        (parse("h(?x)"), parse("g(?x)")),
    ];

    // Pretty print since assert_eq is very hard to read (stdout is only shown on failures)
    pretty_print(&calculated, &expected);

    assert!(to_hashset(calculated) == to_hashset(expected));
}

#[test]
fn no_critical_pairs() {
    let trs = TermRewritingSystem::new(
        vec![
            RewriteRule::new("f(?x)", "g(?x)").unwrap(),
            RewriteRule::new("h(?x)", "k(?x)").unwrap(),
        ]
    );

    let calculated = trs.critical_pairs();
    let expected = vec![];

    // Pretty print since assert_eq is very hard to read (stdout is only shown on failures)
    pretty_print(&calculated, &expected);

    assert!(to_hashset(calculated) == to_hashset(expected));
}