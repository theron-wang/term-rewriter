use std::collections::HashMap;

use crate::{matching::match_term, parser::parse_term, terms::Term};

pub struct RewriteRule {
    lhs: Term,
    rhs: Term,
}

pub struct TermRewritingSystem {
    rules: Vec<RewriteRule>,
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
    fn rewrite_at_root(&self, term: Term) -> (Term, bool) {
        for rule in &self.rules {
            if let Some(subst) = match_term(&rule.lhs, &term) {
                return (apply_sub(&subst, &rule.rhs), true);
            }
        }

        return (term, false);
    }

    /// Rewrites terms through DFS, one step only. If rewrite_at_root succeeds on a term, and its subterms are not looked at, and that rewritten term is returned.
    /// If no rules apply to a term, call rewrite on each of its subterms, until one of them is rewritten successfully. If no rules match, then
    /// this returns (unchanged term, false).
    pub fn rewrite(&self, term: Term) -> (Term, bool) {
        match self.rewrite_at_root(term) {
            (root, true) => (root, true),
            (root, false) => match root {
                Term::Variable(_) => (root, false),
                Term::Function(f, operands) => {
                    let mut modified_operands = Vec::with_capacity(operands.len());
                    let mut any_changed = false;

                    let mut op_iter = operands.into_iter();

                    for op in &mut op_iter {
                        let (rewritten, changed) = self.rewrite(op);
                        modified_operands.push(rewritten);

                        if changed {
                            any_changed = true;
                            break;
                        }
                    }

                    modified_operands.extend(op_iter);

                    (Term::Function(f, modified_operands), any_changed)
                }
            },
        }
    }
}

/// For a map of substitutions (?x -> <term>, ?y -> <term>, ...) and a term, replace all instances of the variables in the map with their
/// substitutions, and return a cloned term.
fn apply_sub(substitutions: &HashMap<String, &Term>, term: &Term) -> Term {
    match term {
        Term::Variable(var) => {
            if let Some(substitution) = substitutions.get(var) {
                (*substitution).clone()
            } else {
                term.clone()
            }
        }
        Term::Function(f, operands) => Term::Function(
            f.to_string(),
            operands
                .into_iter()
                .map(|op| apply_sub(substitutions, op))
                .collect(),
        ),
    }
}
