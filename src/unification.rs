use std::{collections::HashMap, iter::zip};

use crate::{rewriting::apply_sub_single, terms::Term};

/// Performs unification on two terms, finding a map of substitutions that makes two terms equal, specifically the most general unifier (mgu).
/// Will return None if unification is not possible.
pub fn unify(term1: Term, term2: Term) -> Option<HashMap<String, Term>> {
    let mut result = HashMap::new();

    if solve(vec![(term1, term2)], &mut result) {
        Some(result)
    } else {
        None
    }
}

/// Performs the actual unification algorithm, but performed iteratively instead of recursively (see TRaAT §4.7).
fn solve(mut rules: Vec<(Term, Term)>, result: &mut HashMap<String, Term>) -> bool {
    while let Some(rule) = rules.pop() {
        match rule {
            (Term::Variable(var), term) => {
                // Delete
                if let Term::Variable(ref other) = term
                    && var.eq(other)
                {
                    continue;
                } else {
                    // Eliminate
                    if !elim(var, term, &mut rules, result) {
                        return false;
                    }
                }
            }
            (term, Term::Variable(var)) => {
                // Orient + Eliminate
                if !elim(var, term, &mut rules, result) {
                    return false;
                }
            }
            (Term::Function(f, f_ops), Term::Function(g, g_ops)) => {
                // Clash
                if f != g || f_ops.len() != g_ops.len() {
                    return false;
                }

                // Decompose
                rules.extend(zip(f_ops, g_ops));
            }
        }
    }

    true
}

/// Performs the elimination operation in the unification algorithm; adds a mapping of a variable to a term to the result
/// substitution map, and changes all references of that variable to the substituted term.
fn elim(
    var: String,
    term: Term,
    rules: &mut Vec<(Term, Term)>,
    result: &mut HashMap<String, Term>,
) -> bool {
    // Occurs-Check
    if occurs(&var, &term) {
        return false;
    }

    let var_str = var.as_str();

    *rules = rules
        .drain(..)
        .map(|(a, b)| {
            (
                apply_sub_single((var_str, &term), a),
                apply_sub_single((var_str, &term), b),
            )
        })
        .collect();

    *result = result
        .drain()
        .map(|(key, t)| (key, apply_sub_single((var_str, &term), t)))
        .collect();

    result.insert(var, term);

    true
}

/// Checks to see if the variable is in this term.
fn occurs(var: &str, term: &Term) -> bool {
    match term {
        Term::Function(_, ops) => ops.iter().any(|o| occurs(var, o)),
        Term::Variable(name) => var == name,
    }
}
