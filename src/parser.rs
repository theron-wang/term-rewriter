/**
 * We'll use this very simple grammar for this term rewriter.
 *
 * An identifier, in this context, is a string containing strictly digits, letters, and _.
 *    For this program, we'll allow identifiers to start with digits.
 *
 * An operator is a single special character: +, -, *, /
 *
 * Constants <identifier>: a, b, term, c1
 * Variables ?<identifier>: ?x, ?y
 * Functions <identifier|operator>(arg1, arg2, ...): f(a, ?x), myfunction(), +(1, 2)
 */
use crate::syntax::*;

/// Recursively parses a term from an &str. Assumes ASCII-only inputs for simplicity.
pub fn parse_term(term: &str) -> Result<Term, ()> {
    let term = term.trim();

    if term.is_empty() {
        return Err(());
    }

    // If the term contains a (, this term is guaranteed to be a function.
    // Handle variable and constants first, since they're easier
    if !term.contains('(') {
        let mut term_to_validate = term;
        let is_var = term.starts_with('?');

        // Handle variables, but "?" is not a valid variable
        if is_var {
            term_to_validate = &term[1..];
        }

        if term_to_validate.is_empty()
            || term_to_validate
                .chars()
                .any(|c| !c.is_ascii_alphanumeric() && c != '_')
        {
            return Err(());
        }

        return Ok(if is_var {
            Term::Variable(term.to_string())
        } else {
            Term::Constant(term.to_string())
        });
    }

    if term.starts_with('(') {
        return Err(());
    }

    let mut chars = term.chars();

    let mut iden_len = 0;
    let mut must_be_paren = false;

    for term_char in &mut chars {
        if term_char == '(' {
            break;
        }

        iden_len += 1;

        // We reach this when the last character was an operator, but this was not a ( as we expected
        if must_be_paren {
            if term_char.is_ascii_whitespace() {
                continue;
            }
            return Err(());
        }

        if !term_char.is_ascii_alphanumeric() && term_char != '_' {
            match term_char {
                '+' | '-' | '/' | '*' => {
                    must_be_paren = true;
                }
                _ if term_char.is_ascii_whitespace() => must_be_paren = true,
                _ => return Err(()),
            };
        }
    }

    // term_char should now be after that initial (
    let mut operands = Vec::new();

    let mut paren_depth = 1;

    let mut operand_start = iden_len + 1;
    let mut operand_end = operand_start;

    for term_char in &mut chars {
        let mut operand_complete = false;

        if term_char == '(' {
            paren_depth += 1;
        } else if term_char == ')' {
            paren_depth -= 1;

            if paren_depth == 0 {
                operand_complete = true;
            }
        } else if term_char == ',' {
            if paren_depth == 1 {
                operand_complete = true;
            }
        }

        if operand_complete {
            // Don't do anything if this is a no-arg function call
            if operands.is_empty() && paren_depth == 0 && term[operand_start..operand_end].trim().is_empty() {
                break;
            }

            operands.push(parse_term(&term[operand_start..operand_end])?);

            if paren_depth == 0 {
                break;
            }

            operand_end += 1;
            operand_start = operand_end;
        } else {
            operand_end += 1;
        }
    }

    if chars.next().is_some() || paren_depth != 0 {
        return Err(());
    }

    return Ok(Term::Function(
        term[..iden_len].trim().to_string(),
        operands,
    ));
}
