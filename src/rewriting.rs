use std::{borrow::Borrow, collections::HashMap};

use crate::{matching::match_term, parser::parse_term, terms::Term};

pub struct TermRewritingSystem {
    pub(crate) rules: Vec<RewriteRule>,
}

pub struct RewriteRule {
    pub lhs: Term,
    pub rhs: Term,
}

#[derive(PartialEq, Debug)]
pub enum NormalizationResult {
    Normalized(Term),
    LimitExceeded(Term),
}

#[derive(PartialEq, Debug)]
pub enum RewriteResult {
    Rewritten(Term),
    Unchanged(Term),
}

impl RewriteRule {
    pub fn new(lhs: &str, rhs: &str) -> Result<RewriteRule, ()> {
        Ok(RewriteRule {
            lhs: parse_term(lhs)?,
            rhs: parse_term(rhs)?,
        })
    }
}

impl TermRewritingSystem {
    pub fn new(rules: Vec<RewriteRule>) -> TermRewritingSystem {
        TermRewritingSystem { rules }
    }

    /// Rewrites the given term at its root with this TRS using the first matching rule.
    /// If no rules match, then this returns (unchanged term, false).
    fn rewrite_at_root(&self, term: Term) -> RewriteResult {
        for rule in &self.rules {
            if let Some(subst) = match_term(&rule.lhs, &term) {
                return RewriteResult::Rewritten(apply_sub(&subst, rule.rhs.clone()));
            }
        }

        RewriteResult::Unchanged(term)
    }

    /// Rewrites terms through DFS, one step only. If rewrite_at_root succeeds on a term, and its subterms are not looked at, and that rewritten term is returned.
    /// If no rules apply to a term, call rewrite on each of its subterms, until one of them is rewritten successfully. If no rules match, then
    /// this returns (unchanged term, false).
    pub fn rewrite(&self, term: Term) -> RewriteResult {
        match self.rewrite_at_root(term) {
            RewriteResult::Rewritten(root) => RewriteResult::Rewritten(root),
            RewriteResult::Unchanged(root) => match root {
                Term::Variable(_) => RewriteResult::Unchanged(root),
                Term::Function(f, operands) => {
                    let mut modified_operands = Vec::with_capacity(operands.len());
                    let mut any_changed = false;

                    let mut op_iter = operands.into_iter();

                    for op in &mut op_iter {
                        match self.rewrite(op) {
                            RewriteResult::Rewritten(rewritten) => {
                                modified_operands.push(rewritten);
                                any_changed = true;
                                break;
                            }
                            RewriteResult::Unchanged(rewritten) => {
                                modified_operands.push(rewritten);
                            }
                        }
                    }

                    modified_operands.extend(op_iter);

                    let func = Term::Function(f, modified_operands);

                    if any_changed {
                        RewriteResult::Rewritten(func)
                    } else {
                        RewriteResult::Unchanged(func)
                    }
                }
            },
        }
    }

    /// Normalizes a term for the given TRS, with a maximum rewrite count for non-terminating TRSes.
    pub fn normalize(&self, mut term: Term, max_rewrites: usize) -> NormalizationResult {
        let mut norms = 0;
        loop {
            if norms >= max_rewrites {
                return NormalizationResult::LimitExceeded(term);
            }

            match self.rewrite(term) {
                RewriteResult::Unchanged(rewritten_term) => {
                    return NormalizationResult::Normalized(rewritten_term);
                }
                RewriteResult::Rewritten(rewritten_term) => {
                    term = rewritten_term;
                    norms += 1;
                }
            }
        }
    }

    /// Returns true if the two terms are equal (based on their normalizations), given a rewrite limit.
    /// Terms that fail to normalize makes this function return false.
    pub fn are_equal(&self, term1: Term, term2: Term, max_rewrites: usize) -> bool {
        let first = self.normalize(term1, max_rewrites);
        let second = self.normalize(term2, max_rewrites);

        match (first, second) {
            (NormalizationResult::Normalized(first), NormalizationResult::Normalized(second)) => {
                first == second
            }
            _ => false,
        }
    }
}

/// For a map of substitutions (?x -> <term>, ?y -> <term>, ...) and a term, replace all instances of the variables in the map with their
/// substitutions, and return a cloned term.
pub fn apply_sub<T>(substitutions: &HashMap<String, T>, term: Term) -> Term
where
    T: Borrow<Term>,
{
    apply_sub_impl(&|var| substitutions.get(var).map(Borrow::borrow), term)
}

/// For a single substitution (?x -> <term>) and a term, replace all instances of the variable with its substitution, returning a clone.
pub fn apply_sub_single<T>((sub_var, sub): (&str, T), term: Term) -> Term
where
    T: Borrow<Term>,
{
    apply_sub_impl(
        &|var| {
            if var == sub_var {
                Some(sub.borrow())
            } else {
                None
            }
        },
        term,
    )
}

fn apply_sub_impl<'a, T>(substitutions: &T, term: Term) -> Term
where
    T: Fn(&str) -> Option<&'a Term>,
{
    match term {
        Term::Variable(ref var) => {
            if let Some(substitution) = substitutions(var) {
                substitution.clone()
            } else {
                term
            }
        }
        Term::Function(f, operands) => Term::Function(
            f,
            operands
                .into_iter()
                .map(|op| apply_sub_impl(substitutions, op))
                .collect(),
        ),
    }
}
