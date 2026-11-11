use term_rewriter::{
    parser::parse_term,
    rewriting::{RewriteRule, TermRewritingSystem},
    terms::Term,
};

/// Use instead of parse_term for brevity (to avoid having to call unwrap() each time)
pub fn parse(term: &str) -> Term {
    parse_term(term).unwrap()
}

/// Terminating, non-confluent TRS representing boolean operations.
/// Uses the following rules:
/// 1. `not(not(?x))` -> `?x`
/// 2. `not(and(?x, ?y))` -> `or(not(?x), not(?y))`
/// 3. `not(or(?x, ?y))` -> `and(not(?x), not(?y))`
/// 4. `or(and(?x, ?y), ?z)` -> `and(or(?x, ?z), or(?y, ?z))`
/// 5. `or(?x, and(?y, ?z))` -> `and(or(?x, ?y), or(?x, ?z))`
pub fn get_boolean_trs() -> TermRewritingSystem {
    /*
    # Termination

    Informally, this is terminating under this reduction order (using lexicographic path ordering; see Term Rewriting and All That § 5.4.2):
            Using: not > or > and

    1. Since ?x ∈ Var(not(not(?x))), then ?x < not(not(?x)). (LPO1)
    2. Since not > or (LPO2b: s > t iff f > g and s > all direct subterms of t):
        2a. Check if not(and(?x, ?y)) > not(?x) and also not(and(?x, ?y)) > not(?y) with i = 1 (LPO2c: s > t iff f = g and all direct subterms up to a
            certain i are equal, and si > ti, and all direct subterms of t are less than s):
                * By LPO1, we can clearly see and(?x, ?y) > ?x and and(?x, ?y) > ?y.
    3. This is a similar proof to #2.
    4. Since or > and (LPO2b):
        4a. Now, we need to check if or(and(?x, ?y), ?z) > or(?x, ?z) with i = 1 (LPO2c)
                * and(?x, ?y) > ?x (by LPO1)
                * ?x, ?z are dominated by or(and(?x, ?y), ?z) since they are variables within s (by LPO1)
        4b. The check for or(and(?x, ?y), ?z) > or(?y, ?z) is identical to (4a).
    5. Similar to #4, but with i = 2 instead of i = 1.

    => Since all rewrite rules are strictly decreasing under LPO, the TRS is terminating.

    # Local Confluence

    If we can prove local confluence, we can use Newman's Lemma to prove confluence, since we already proved the TRS is terminating.

    Overlapping terms:
    - R1 and R2: not(not(and(?x, ?y)))
        * Applying R1 --> and(?x, ?y)
        * Applying R2 --> not(or(not(?x), not(?y))) --> and(not(not(?x)), not(not(?y))) --> and(?x, ?y)
        => R1 and R2 are joinable
    - R1 and R3: not(not(or(?x, ?y)))
        * Applying R1 --> or(?x, ?y)
        * Applying R2 --> not(and(not(?x), not(?y))) --> or(not(not(?x)), not(not(?y))) --> or(?x, ?y)
        => R1 and R3 are joinable
    - R3 and R4: not(or(and(?x, ?y), ?z))
        * Applying R3 --> and(not(and(?x, ?y)), not(?z)) --> and(or(not(?x), not(?y)), not(?z))
        * Applying R4: not(and(or(?x, ?z), or(?y, ?z))) --> or(not(or(?x, ?z)), not(or(?y, ?z))) --> or(and(not(?x), not(?z)), and(not(?y), not(?z)))
            --> and(or(not(?x), and(not(?y), not(?z))), or(not(?z), and(not(?y), not(?z))))
            --> and(and(or(not(?x), not(?y)), or(not(?x), not(?z))), and(or(not(?z), not(?y)), or(not(?z), not(?z))))
        => R3 and R4 are not joinable

    Since not all critical pairs are joinable, this TRS is not locally confluent, and thus not confluent.
    */

    TermRewritingSystem::new(vec![
        // See https:en.wikipedia.org/wiki/Rewriting#Logic
        RewriteRule::new("not(not(?x))", "?x").unwrap(),
        // DeMorgan's Laws
        RewriteRule::new("not(and(?x, ?y))", "or(not(?x), not(?y))").unwrap(),
        RewriteRule::new("not(or(?x, ?y))", "and(not(?x), not(?y))").unwrap(),
        // Others
        RewriteRule::new("or(and(?x, ?y), ?z)", "and(or(?x, ?z), or(?y, ?z))").unwrap(),
        RewriteRule::new("or(?x, and(?y, ?z))", "and(or(?x, ?y), or(?x, ?z))").unwrap(),
    ])
}

/// Terminating and confluent TRS representing arithmetic with addition and multiplication.
/// Uses the following rules:
/// 1. `+(0, ?x)` -> `?x`
/// 2. `*(0, ?x)` -> `0`
/// 3. `+(?x, s(?y)`) -> `s(+(?x, ?y))`
/// 4. `*(?x, s(?y))` -> `+(?x, *(?x, ?y))`
pub fn get_arithmetic_trs() -> TermRewritingSystem {
    /*
    # Termination

    Let * > + > s > 0.
    1. +(0, ?x) > ?x since ?x ∈ Var(+(0, ?x)) (LPO1)
    2. *(0, ?x) > 0 since * > 0 and 0, ?x are greater than 0's arguments since it has none (LPO2b)
    3. Since + > s: (LPO2b)
        3a. Check +(?x, s(?y)) > +(?x, ?y) with i = 2 (LPO2c)
            - +(?x, s(?y)) > ?x and +(?x, s(?y)) > ?y (LPO1)
            - s(?y) > ?y (LPO1)
    4. Since * > +: (LPO2b)
        4a. *(?x, s(?y)) > ?x (LPO1)
        4b. *(?x, s(?y)) > *(?x, ?y); notice * = * (LPO2c)
            - *(?x, s(?y)) > ?x, ?y (LPO1)
            - Using i = 2, s(?y) > ?y (LPO1)

    => Since all rewrite rules strictly decrease under LPO, this TRS is terminating.

    # Confluence

    Overlapping terms:
    - R1 and R3: +(0, s(?y)):
        * Applying R1: s(?y)
        * Applying R3: s(+(0, ?y)) --> s(?y)
        => R1 and R3 are joinable.
    - R2 and R4: *(0, s(?y)):
        * Applying R2: 0
        * Applying R4: +(0, *(0, ?y)) --> *(0, 0) --> 0
        => R2 and R4 are joinable.

    => Since all the critical pairs are joinable, this TRS is locally confluent.
    => Since this TRS is terminating and locally confluent, this TRS is confluent, by Newman's Lemma.
    */
    TermRewritingSystem::new(vec![
        // See https:en.wikipedia.org/wiki/Rewriting#Arithmetic
        RewriteRule::new("+(0, ?x)", "?x").unwrap(),
        RewriteRule::new("*(0, ?x)", "0").unwrap(),
        // x + (y + 1) == (x + y) + 1
        RewriteRule::new("+(?x, s(?y))", "s(+(?x, ?y))").unwrap(),
        // x * (y + 1) == x + x * y
        RewriteRule::new("*(?x, s(?y))", "+(?x, *(?x, ?y))").unwrap(),
    ])
}

/// Gets a simple term-rewriting system that is confluent but non-terminating.
/// Uses the following rules:
/// 1. `f(?x, a)` -> `g(?x)`
/// 2. `f(?x, a)` -> `h(?x)`
/// 3. `g(?x)` -> `r(?x)`
/// 4. `h(?x)` -> `r(?x)`
/// 5. `r(?x)` -> `f(?x, a)`
pub fn get_confluent_nonterminating_trs() -> TermRewritingSystem {
    //             +-- g(?x) --+
    // f(?x, a) -- |           | -- r(?x) -- f(?x, a)
    //             +-- h(?x) --+
    // By inspection, it is clear that this TRS is indeed confluent (they end back at f(?x, a)) but non-terminating, since
    // f(?x, a) can be infinitely rewritten as itself.
    TermRewritingSystem::new(vec![
        RewriteRule::new("f(?x, a)", "g(?x)").unwrap(),
        RewriteRule::new("f(?x, a)", "h(?x)").unwrap(),
        RewriteRule::new("g(?x)", "r(?x)").unwrap(),
        RewriteRule::new("h(?x)", "r(?x)").unwrap(),
        RewriteRule::new("r(?x)", "f(?x, a)").unwrap(),
    ])
}
