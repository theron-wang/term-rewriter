use term_rewriter::{parser::parse_term, syntax::Term};

#[test]
fn assert_valid_constants() {
    assert_eq!(parse_term("a").unwrap(), Term::Constant(String::from("a")));
    assert_eq!(
        parse_term("_abcABC_123").unwrap(),
        Term::Constant(String::from("_abcABC_123"))
    );
    assert_eq!(parse_term("1").unwrap(), Term::Constant(String::from("1")));
    assert_eq!(
        parse_term("c1").unwrap(),
        Term::Constant(String::from("c1"))
    );
    assert_eq!(
        parse_term("term").unwrap(),
        Term::Constant(String::from("term"))
    );
    assert_eq!(
        parse_term("  my_term   ").unwrap(),
        Term::Constant(String::from("my_term"))
    );
}

#[test]
fn assert_malformed_constants() {
    assert!(parse_term("").is_err());
    assert!(parse_term("a b").is_err());
    assert!(parse_term("+").is_err());
    assert!(parse_term("a-b").is_err());
    assert!(parse_term("!var").is_err());
}

#[test]
fn assert_valid_variables() {
    assert_eq!(
        parse_term("?a").unwrap(),
        Term::Variable(String::from("?a"))
    );
    assert_eq!(
        parse_term("?_abcABC_123").unwrap(),
        Term::Variable(String::from("?_abcABC_123"))
    );
    assert_eq!(
        parse_term("?1").unwrap(),
        Term::Variable(String::from("?1"))
    );
    assert_eq!(
        parse_term("?c1").unwrap(),
        Term::Variable(String::from("?c1"))
    );
    assert_eq!(
        parse_term("?term").unwrap(),
        Term::Variable(String::from("?term"))
    );
    assert_eq!(
        parse_term("  ?my_term   ").unwrap(),
        Term::Variable(String::from("?my_term"))
    );
}

#[test]
fn assert_malformed_variables() {
    assert!(parse_term("?").is_err());
    assert!(parse_term("?a ?b").is_err());
    assert!(parse_term("?+").is_err());
    assert!(parse_term("?a-?b").is_err());
    assert!(parse_term("?!var").is_err());
    assert!(parse_term("??var").is_err());
}

#[test]
fn assert_valid_functions() {
    assert_eq!(
        parse_term("f()").unwrap(),
        Term::Function(String::from("f"), vec![])
    );
    assert_eq!(
        parse_term("f( )").unwrap(),
        Term::Function(String::from("f"), vec![])
    );
    assert_eq!(
        parse_term("-()").unwrap(),
        Term::Function(String::from("-"), vec![])
    );
    assert_eq!(
        parse_term("/()").unwrap(),
        Term::Function(String::from("/"), vec![])
    );
    assert_eq!(
        parse_term("*()").unwrap(),
        Term::Function(String::from("*"), vec![])
    );
    assert_eq!(
        parse_term("+(1, 2)").unwrap(),
        Term::Function(
            String::from("+"),
            vec![
                Term::Constant(String::from("1")),
                Term::Constant(String::from("2"))
            ]
        )
    );
    assert_eq!(
        parse_term("my_func(?x, const)").unwrap(),
        Term::Function(
            String::from("my_func"),
            vec![
                Term::Variable(String::from("?x")),
                Term::Constant(String::from("const"))
            ]
        )
    );
    assert_eq!(
        parse_term("my_func(?x)").unwrap(),
        Term::Function(
            String::from("my_func"),
            vec![Term::Variable(String::from("?x"))]
        )
    );
    assert_eq!(
        parse_term("my_func(nested(+(1), a))").unwrap(),
        Term::Function(
            String::from("my_func"),
            vec![Term::Function(
                String::from("nested"),
                vec![
                    Term::Function(String::from("+"), vec![Term::Constant(String::from("1"))]),
                    Term::Constant(String::from("a"))
                ]
            )]
        )
    );
    assert_eq!(
        parse_term("  my_func   (   ?x, a     )   ").unwrap(),
        Term::Function(
            String::from("my_func"),
            vec![
                Term::Variable(String::from("?x")),
                Term::Constant(String::from("a"))
            ]
        )
    );
}

#[test]
fn assert_malformed_functions() {
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
