use term_rewriter::{parser::parse_term, terms::Term};

#[test]
fn valid_constants() {
    assert_eq!(
        parse_term("a").unwrap(),
        Term::Function("a".to_string(), vec![])
    );
    assert_eq!(
        parse_term("_abcABC_123").unwrap(),
        Term::Function("_abcABC_123".to_string(), vec![])
    );
    assert_eq!(
        parse_term("1").unwrap(),
        Term::Function("1".to_string(), vec![])
    );
    assert_eq!(
        parse_term("c1").unwrap(),
        Term::Function("c1".to_string(), vec![])
    );
    assert_eq!(
        parse_term("term").unwrap(),
        Term::Function("term".to_string(), vec![])
    );
    assert_eq!(
        parse_term("  my_term   ").unwrap(),
        Term::Function("my_term".to_string(), vec![])
    );
}

#[test]
fn malformed_constants() {
    assert!(parse_term("").is_err());
    assert!(parse_term("a b").is_err());
    assert!(parse_term("+").is_err());
    assert!(parse_term("a-b").is_err());
    assert!(parse_term("!var").is_err());
}

#[test]
fn valid_variables() {
    assert_eq!(parse_term("?a").unwrap(), Term::Variable("?a".to_string()));
    assert_eq!(
        parse_term("?_abcABC_123").unwrap(),
        Term::Variable("?_abcABC_123".to_string())
    );
    assert_eq!(parse_term("?1").unwrap(), Term::Variable("?1".to_string()));
    assert_eq!(
        parse_term("?c1").unwrap(),
        Term::Variable("?c1".to_string())
    );
    assert_eq!(
        parse_term("?term").unwrap(),
        Term::Variable("?term".to_string())
    );
    assert_eq!(
        parse_term("  ?my_term   ").unwrap(),
        Term::Variable("?my_term".to_string())
    );
}

#[test]
fn malformed_variables() {
    assert!(parse_term("?").is_err());
    assert!(parse_term("?a ?b").is_err());
    assert!(parse_term("?+").is_err());
    assert!(parse_term("?a-?b").is_err());
    assert!(parse_term("?!var").is_err());
    assert!(parse_term("??var").is_err());
}

#[test]
fn valid_functions() {
    assert_eq!(
        parse_term("f()").unwrap(),
        Term::Function("f".to_string(), vec![])
    );
    assert_eq!(
        parse_term("f( )").unwrap(),
        Term::Function("f".to_string(), vec![])
    );
    assert_eq!(
        parse_term("-()").unwrap(),
        Term::Function("-".to_string(), vec![])
    );
    assert_eq!(
        parse_term("/()").unwrap(),
        Term::Function("/".to_string(), vec![])
    );
    assert_eq!(
        parse_term("*()").unwrap(),
        Term::Function("*".to_string(), vec![])
    );
    assert_eq!(
        parse_term("+(1, 2)").unwrap(),
        Term::Function(
            "+".to_string(),
            vec![
                Term::Function("1".to_string(), vec![]),
                Term::Function("2".to_string(), vec![])
            ]
        )
    );
    assert_eq!(
        parse_term("my_func(?x, const)").unwrap(),
        Term::Function(
            "my_func".to_string(),
            vec![
                Term::Variable("?x".to_string()),
                Term::Function("const".to_string(), vec![])
            ]
        )
    );
    assert_eq!(
        parse_term("my_func(?x)").unwrap(),
        Term::Function(
            "my_func".to_string(),
            vec![Term::Variable("?x".to_string())]
        )
    );
    assert_eq!(
        parse_term("my_func(nested(+(1), a))").unwrap(),
        Term::Function(
            "my_func".to_string(),
            vec![Term::Function(
                "nested".to_string(),
                vec![
                    Term::Function(
                        "+".to_string(),
                        vec![Term::Function("1".to_string(), vec![])]
                    ),
                    Term::Function("a".to_string(), vec![])
                ]
            )]
        )
    );
    assert_eq!(
        parse_term("  my_func   (   ?x, a     )   ").unwrap(),
        Term::Function(
            "my_func".to_string(),
            vec![
                Term::Variable("?x".to_string()),
                Term::Function("a".to_string(), vec![])
            ]
        )
    );
}

#[test]
fn malformed_functions() {
    assert!(parse_term("f(").is_err());
    assert!(parse_term("f())").is_err());
    assert!(parse_term("f(,)").is_err());
    assert!(parse_term("f(a,)").is_err());
    assert!(parse_term("f(,a)").is_err());
    assert!(parse_term("f(g(a), b)a").is_err());
    assert!(parse_term("++()").is_err());
    assert!(parse_term("?f()").is_err());
    assert!(parse_term("f g()").is_err());
    assert!(parse_term("()").is_err());
}
