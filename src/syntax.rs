use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Term {
    Variable(String),
    Function(String, Vec<Term>),
}

pub struct RewriteRule {
    pub lhs: Term,
    pub rhs: Term,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(name) => write!(f, "{}", name),
            Term::Function(name, operands) if operands.is_empty() => write!(f, "{}", name),
            Term::Function(name, operands) => {
                write!(f, "{name}(")?;
                for (i, operand) in operands.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    operand.fmt(f)?;
                }
                write!(f, ")")
            }
        }
    }
}
