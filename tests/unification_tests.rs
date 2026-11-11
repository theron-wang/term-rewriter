use std::collections::HashMap;
use term_rewriter::{terms::Term, unification::unify};

use crate::common::parse;

#[allow(dead_code)]
mod common;

/// Helper method to simplify the creation of hashmaps in assert_eq! calls in this testing file.
fn map_to<'a>(mappings: Vec<(&str, Term)>) -> Option<HashMap<String, Term>> {
    return Some(
        mappings
            .into_iter()
            .map(|(a, b)| (a.to_string(), b))
            .collect(),
    );
}

#[test]
fn unification_simple() {
    assert_eq!(
        unify(parse("?x"), parse("a")),
        map_to(vec![("?x", parse("a"))])
    );

    assert_eq!(
        unify(parse("a"), parse("?x")),
        map_to(vec![("?x", parse("a"))])
    );
}

#[test]
fn unification_identity() {
    assert_eq!(
        unify(parse("f(a, ?x, b)"), parse("f(a, ?x, b)")),
        map_to(vec![])
    );
}

#[test]
fn unification_nested() {
    assert_eq!(
        unify(
            parse("f(?x, g(?y, h(?z)), a)"),
            parse("f(g(b), g(?x, h(f(?x, g(?y)))), a)")
        ),
        map_to(vec![
            ("?x", parse("g(b)")),
            ("?y", parse("g(b)")),
            ("?z", parse("f(g(b), g(g(b)))"))
        ])
    );
}

#[test]
fn unification_with_deletion() {
    assert_eq!(
        unify(parse("f(?x, ?y)"), parse("f(?y, ?y)")),
        map_to(vec![("?x", parse("?y"))])
    );
}

#[test]
fn unification_decompose_and_elimination() {
    assert_eq!(
        unify(
            parse("f(?x, f(a, ?y), b, g(g(?z)))"),
            parse("f(a, f(a, ?x), b, g(g(g(?y))))")
        ),
        map_to(vec![
            ("?x", parse("a")),
            ("?y", parse("a")),
            ("?z", parse("g(a)"))
        ])
    );
}

#[test]
fn unification_fails_with_occurs_check() {
    assert!(unify(parse("?x"), parse("f(?x)")).is_none());
    assert!(unify(parse("?x"), parse("?x")).is_some());
    assert!(unify(parse("f(?x)"), parse("?x")).is_none());
    assert!(unify(parse("f(?x)"), parse("f(g(?x))")).is_none());
    assert!(unify(parse("f(?x, ?y)"), parse("f(g(?y), g(?x))")).is_none());
}

#[test]
fn unification_fails_with_clash() {
    assert!(unify(parse("f"), parse("f(?x)")).is_none());
    assert!(unify(parse("g(a)"), parse("f(?x)")).is_none());
    assert!(unify(parse("f(a, ?x)"), parse("f(?x)")).is_none());
    assert!(unify(parse("f(a, g(?x))"), parse("f(?x, g(a, b))")).is_none());
}

#[test]
fn unification_fails_with_mismatch() {
    assert!(unify(parse("f(?x, ?x)"), parse("f(a, b)")).is_none());
    assert!(unify(parse("f(?x, g(?x))"), parse("f(a, g(b))")).is_none());
}
