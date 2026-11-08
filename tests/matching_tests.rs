use std::collections::HashMap;
use term_rewriter::{matching::match_term, parser::parse_term, terms::Term};

/// Helper method to simplify the creation of hashmaps in assert_eq! calls in this testing file.
fn map_to<'a>(mappings: Vec<(&str, &'a Term)>) -> Option<HashMap<String, &'a Term>> {
    return Some(mappings.iter().map(|(a, b)| (a.to_string(), *b)).collect());
}

// Use instead of parse_term for brevity (to avoid unwrapping)
fn parse(term: &str) -> Term {
    parse_term(term).unwrap()
}

#[test]
fn assert_match_simple_var_to_const() {
    let pattern = parse("?x");
    let term = parse("a");

    let result = match_term(&pattern, &term);
    assert_eq!(result, map_to(vec![("?x", &term)]));
}

#[test]
fn assert_match_simple_var_to_func() {
    let pattern = parse("?x");
    let term = parse("f(a)");

    let result = match_term(&pattern, &term);
    assert_eq!(result, map_to(vec![("?x", &term)]));
}

#[test]
fn assert_match_simple_var_to_var() {
    let pattern = parse("?x");
    let term = parse("?y");

    let result = match_term(&pattern, &term);
    assert_eq!(result, map_to(vec![("?x", &term)]));
}

#[test]
fn assert_match_self() {
    let pattern = parse("?x");

    let result = match_term(&pattern, &pattern);
    assert_eq!(result, map_to(vec![("?x", &pattern)]));
}

#[test]
fn assert_match_self_constants() {
    let pattern = parse("a");

    let result = match_term(&pattern, &pattern);
    assert_eq!(result, Some(HashMap::new()));
}

#[test]
fn assert_match_self_no_vars() {
    let pattern = parse("f(a, g(b))");

    let result = match_term(&pattern, &pattern);
    assert_eq!(result, Some(HashMap::new()));
}

#[test]
fn assert_match_multiple_args() {
    let pattern = parse("f(a, ?x, ?y, b)");
    let term = parse("f(a, g(a), h(a, ?x), b)");

    let result = match_term(&pattern, &term);
    assert_eq!(
        result,
        map_to(vec![("?x", &parse("g(a)")), ("?y", &parse("h(a, ?x)"))])
    );
}

#[test]
fn assert_match_repeated_vars() {
    let pattern = parse("f(a, ?x, ?x)");
    let term = parse("f(a, g(?y), g(?y))");

    let result = match_term(&pattern, &term);
    assert_eq!(result, map_to(vec![("?x", &parse("g(?y)"))]));
}

#[test]
fn assert_match_repeated_vars_different_depths() {
    let pattern = parse("f(a, ?x, g(?x))");
    let term = parse("f(a, g(?y), g(g(?y)))");

    let result = match_term(&pattern, &term);
    assert_eq!(result, map_to(vec![("?x", &parse("g(?y)"))]));
}

#[test]
fn assert_match_deep_nesting() {
    let pattern = parse("f(?x, g(h(?x), ?y), g(h(f1(?z, f2(?x)))))");
    let term = parse(
        "f(?x, g(h(?x), ?y), g(h(f1(?z, f2(?x)))))"
            .replace("?x", "g(g(a))")
            .replace("?y", "f(g(?x))")
            .replace("?z", "a")
            .as_str(),
    );

    let result = match_term(&pattern, &term);
    assert_eq!(
        result,
        map_to(vec![
            ("?x", &parse("g(g(a))")),
            ("?y", &parse("f(g(?x))")),
            ("?z", &parse("a"))
        ])
    );
}

#[test]
fn assert_reject_constant_to_variable() {
    let pattern = parse("a");
    let term = parse("?x");

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_function_to_variable() {
    let pattern = parse("g(?x)");
    let term = parse("?x");

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_mismatching_arity() {
    let pattern = parse("f(?x)");
    let term = parse("f(a, b)");

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_mismatching_function() {
    let pattern = parse("f(?x)");
    let term = parse("g(a)");

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_conflicting_matches() {
    let pattern = parse("f(?x, ?x)");
    let term = parse("f(a, b)");

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_conflicting_matches_complex() {
    let pattern = parse("f(?x, g(?y), h(g(?x), f(f(f(?y)))))");
    let term = parse(
        "f(?x, g(?y), h(g(?x), f(f(f(?y)))))"
            .replace("?x", "f(g(h(f(g(h(a))))))")
            .replace("f(?y)", "f(a)")
            .replace("?y", "f(b)")
            .as_str(),
    );

    assert!(match_term(&pattern, &term).is_none());
}

#[test]
fn assert_reject_mismatching_constants() {
    let pattern = parse("f(a, h(b, g(c)))");
    let term = parse("f(a, h(b, g(d)))");

    assert!(match_term(&pattern, &term).is_none());
}
