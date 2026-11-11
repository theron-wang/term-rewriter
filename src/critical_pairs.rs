use crate::{
    rewriting::{RewriteRule, TermRewritingSystem, apply_sub},
    terms::Term,
    unification::unify,
};

impl TermRewritingSystem {
    /// Gets all the critical pairs of a given TRS.
    pub fn critical_pairs(&self) -> Vec<(Term, Term)> {
        self.rules
            .iter()
            .map(|rule| self.critical_pairs_for_single_pair((rule.lhs.clone(), rule.rhs.clone())))
            .flatten()
            .collect()
    }

    /// In the given TRS, find all critical pairs for the RHS in subterms of the LHS.
    fn critical_pairs_for_single_pair(&self, (lhs, rhs): (Term, Term)) -> Vec<(Term, Term)> {
        fn handle_cps_for_current(
            context: &dyn Fn(Term) -> Term,
            lhs_subterm: &Term,
            rhs: &Term,
            trs: &TermRewritingSystem,
        ) -> Vec<(Term, Term)> {
            match lhs_subterm {
                // Variables are not very useful here, as its unification will just give itself -> rhs (which will not yield a critical pair)
                Term::Variable(_) => vec![],

                // For each rule in the TRS, calculate all critical pairs with this current lhs subterm and its rhs. Then, we want to
                // calculate all the critical pairs for each subterm of the current lhs subterm.
                Term::Function(_, _) => trs
                    .rules
                    .iter()
                    .map(|rule| {
                        unify_and_substitute_lhs_subterm_and_arbitrary_rule_for_cp(
                            &context,
                            &lhs_subterm,
                            &rhs,
                            rule,
                        )
                    })
                    .filter_map(|x| x)
                    .chain(handle_inner_cps_for_current(context, lhs_subterm, rhs, trs))
                    .collect(),
            }
        }

        fn handle_inner_cps_for_current(
            context: &dyn Fn(Term) -> Term,
            term: &Term,
            rhs: &Term,
            trs: &TermRewritingSystem,
        ) -> Vec<(Term, Term)> {
            let Term::Function(f, operands) = term else {
                return vec![];
            };

            let mut critical_pairs = Vec::new();

            for (i, current) in operands.iter().enumerate() {
                let context_for_child_term = |s| {
                    let mut new_operands = Vec::with_capacity(operands.len());

                    new_operands.extend_from_slice(&operands[..i]);
                    new_operands.push(s);
                    new_operands.extend_from_slice(&operands[i + 1..]);

                    context(Term::Function(f.to_string(), new_operands))
                };

                critical_pairs.extend(handle_cps_for_current(
                    &context_for_child_term,
                    current,
                    rhs,
                    trs,
                ));
            }

            critical_pairs
        }

        // Use ! as a suffix because the parser doesn't allow !, so we know that these renamed variables will be different
        // than those in the rewrite rules.
        handle_cps_for_current(&|t| t, &rename(lhs, "!"), &rename(rhs, "!"), self)
    }
}

/// Given a subterm of some rule's LHS, its RHS, and some other rule from the TRS, calculate a critical pair (if it exists).
/// It does so by unifying the LHS with the other rule's LHS, and if that succeeds, then returns a tuple of the
/// substitution of the first RHS and the substitution of the second RHS in the context of the first LHS.
/// i.e., applies two different rewrite rules to some subterm of a rule's LHS.
fn unify_and_substitute_lhs_subterm_and_arbitrary_rule_for_cp<T>(
    context: &T,
    subterm: &Term,
    rhs: &Term,
    rule: &RewriteRule,
) -> Option<(Term, Term)>
where
    T: Fn(Term) -> Term,
{
    match unify(subterm.clone(), rule.lhs.clone()) {
        None => None,
        Some(subst) => {
            let a = undo_rename(apply_sub(&subst, rhs.clone()), "!");
            let b = undo_rename(apply_sub(&subst, context(rule.rhs.clone())), "!");

            if a == b { None } else { Some((a, b)) }
        }
    }
}

/// Renames all the variables in a given term by adding a suffix.
fn rename(term: Term, suffix: &str) -> Term {
    match term {
        Term::Variable(var) => Term::Variable(var + suffix),
        Term::Function(name, operands) => Term::Function(
            name,
            operands
                .into_iter()
                .map(|term| rename(term, suffix))
                .collect(),
        ),
    }
}

/// Undoes a rename for all the variables in a given term by removing a suffix.
fn undo_rename(term: Term, suffix: &str) -> Term {
    match term {
        Term::Variable(var) => Term::Variable(match var.strip_suffix(suffix) {
            Some(str) => str.to_string(),
            None => var,
        }),
        Term::Function(name, operands) => Term::Function(
            name,
            operands
                .into_iter()
                .map(|term| undo_rename(term, suffix))
                .collect(),
        ),
    }
}
