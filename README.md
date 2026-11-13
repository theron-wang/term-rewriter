# Term Rewriter

A term rewriter implemented in Rust, supporting term parsing, pattern matching, term rewriting, normalization, unification, and critical pair generation.

This is an educational project based on the theory and algorithms from _Term Rewriting and All That_.

## Setup

If you already have Rust installed, you can simply run `cargo test` to run all tests. There are no dependencies required. Note that this project is not runnable as a standalone.

## Project Structure

The implementation for the term rewriter can be found in `src/`. Tests demonstrating each feature can be found in `tests/`. For some sample TRSes and their respective proofs for confluence and termination, please check out `tests/common/mod.rs`.

## Features

### Term Parsing

This project uses a simple grammar to parse text into terms. There are two types of terms: functions and variables. Functions can be of arity 0 or greater; a "constant" is simply a function of arity 0.

Functions and variables are parsed largely based on _identifiers_; that is, an alphanumeric string (including `_`) with no whitespace (identifiers can start with digits). Functions also use _operators_, which are `+`, `-`, `*`, and `/`. Functions with no parentheses are assumed to have no subterms.

```
<term>: <function>|<variable>
<variable>: ?[identifier]
<function>: [identifier|operator] | [identifier|operator](<term>, <term>, ...)
```

### Pattern Matching

Given a pattern `p` and a term `t`, matching finds a substitution such that `σp = t`. That is, the implementation finds a `HashMap` of variables to terms such that applying the map to `p` will result in `t`.

### Term Rewriting

The current term rewriter has two distinct rewrite methods: `rewrite` and `rewrite_all`.

#### `rewrite`

This rewrite method applies at most one rewrite on each call. It uses a root-first approach -- i.e., the root is checked first against all rules, and then the subterms recursively. As soon as the root/any subterm matches a rule's left-hand side, the right-hand side of the rule (with the substitution mapping from pattern matching applied) is used to overwrite the matching term.

#### `rewrite_all`

This rewrite method applies all applicable rewrites on a term, returning it in a vector. The order of the rewritten terms is determined by traversal from the root, and from left to right. Each term in the resulting vector has one single rewrite applied.

### Normalization

Normalization is a loop of rewrites until no more rules can be matched. There is a rewrite limit in this term rewriter that allows for inconclusive termination in the case of non-terminating TRSes, which `normalize` does not support, but `normalize_all` does. `normalize_all` finds the normal form in a confluent TRS, and all normal forms in a nonconfluent TRS.

### Unification

Unification finds a substitution such that `σs = σt`. Compared to pattern matching, the left-hand term is no longer treated as static. This implementation finds a most general unifier (mgu), using four operations applied until none can no longer be applied:
- Deletion: deleting trivial substitutions (`?x` -> `?x`)
- Orient: moving variables to the LHS (`t` -> `?x` becomes `?x` -> `t`)
- Decompose: solving equality relationships based on subterms (`f(t1, t2, ...)` = `f(s1, s2, ...)` becomes `t1` = `s1`, `t2` = `s2`, ...)
- Eliminate: once a variable's value is found, then replace all instances of that variable with the discovered mapping (if we find `x` -> `a`, then replace `?x` such that `f(?x)` becomes `f(a)`, etc.)

Two auxiliary checks are also performed to return a unification failure early:
- Clash: mismatching functions or arities (`f` = `g`, `f(t)` = `f(t1, t2)`)
- Occurs-Check: if `?x` -> `s`, then `?x ∈ Var(s)` means that unification is not possible

### Critical Pair Generation

Critical pairs can be useful for proving local confluence, which can be used to prove confluence if the TRS is terminating, by Newman's Lemma. A critical pair can be found through overlapping terms -- a term in which two left-hand sides of the TRS can match. The distinct terms after applying each matching rewrite gives you a critical pair.

In short, the algorithm works as such. Each individual rule requires renames of all its variables to unique names to avoid conflicts with other rules (since we start with the assumption that all rules are independent), and we unify the left-hand side and its subterms with other rules' left-hand sides, and successful unification substitutions are applied to the original rule's right-hand side and the other rule's right-hand side to create a critical pair. Contexts are preserved to ensure that the original structure is also preserved.

## Usage

Construct a new TRS using the following:

```rust
use term_rewriter::{RewriteRule, TermRewritingSystem};
let trs = TermRewritingSystem::new(vec![
    RewriteRule::new("<lhs>", "<rhs>").unwrap(),
    RewriteRule::new("<lhs>", "<rhs>").unwrap(),
    ...
]);
```

To use each feature, you should call the following, assuming `trs` is of type `TermRewritingSystem`:

```rust
use term_rewriter::terms::Term;

// Parsing
let parsed: Result<Term, ()> = term_rewriter::parser::parse_term("term");

// Pattern Matching
let (pattern, term): (Term, Term) = ...;
let matched: Option<HashMap<String, &Term>> = term_rewriter::matching::match_term(pattern_term);

// Term Rewriting
let rewritten: RewriteResult = trs.rewrite(term);
let all_rewrites: Vec<Term> = trs.rewrite_all(&term);

// Normalization
let max_rewrites: usize = ...;
let normalized: NormalizationResult = trs.normalize(term, max_rewrites);
let (found_all_norms, found_normalizations): (bool, HashSet<Term>) = trs.normalize_all(term, max_rewrites);

// Unification
let (term1, term2): (Term, Term) = ...;
let unification: Option<HashMap<String, Term>> = term_rewriter::unification::unify(term1, term2);

// Critical Pairs
let critical_pairs: Vec<(Term, Term)> = trs.critical_pairs();
```

## References

Baader, F., & Nipkow, T. (1998). [_Term Rewriting and All That_](https://doi.org/10.1017/CBO9781139172752). Cambridge University Press.

[The Rust Programming Language](https://doc.rust-lang.org/book/) - Rust language documentation and learning resource.