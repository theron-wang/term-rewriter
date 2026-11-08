use std::collections::HashMap;
use std::iter::zip;

use crate::syntax::Term;

/// Returns a mapping between variables in the pattern, to terms in the provided term. Returns None when a match is impossible.
pub fn match_term<'a>(pattern: &Term, term: &'a Term) -> Option<HashMap<String, &'a Term>> {
    let mut result = HashMap::new();

    match match_term_impl(pattern, term, &mut result) {
        Ok(_) => Some(result),
        Err(_) => None,
    }
}

fn match_term_impl<'a>(
    pattern: &Term,
    term: &'a Term,
    substitutions: &mut HashMap<String, &'a Term>,
) -> Result<(), ()> {
    match (pattern, term) {
        (Term::Variable(a), _) => {
            if let Some(existing_sub) = substitutions.get(a) {
                if *existing_sub != term {
                    return Err(());
                }

                return Ok(());
            }

            substitutions.insert(a.to_string(), term);
            return Ok(());
        }
        (Term::Function(f, f_args), Term::Function(g, g_args)) => {
            if f != g || f_args.len() != g_args.len() {
                return Err(());
            }

            for (p, t) in zip(f_args, g_args) {
                match_term_impl(p, t, substitutions)?;
            }

            return Ok(());
        }
        (_, _) => return Err(()),
    }
}
