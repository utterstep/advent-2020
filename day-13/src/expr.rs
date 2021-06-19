#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Expression {
    Var,
    Const(i64),
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
}

impl Expression {
    #[allow(dead_code)]
    pub fn reduce(&self) -> Self {
        macro_rules! partition_consts {
            ($e: expr) => {
                $e.iter()
                    .map(Self::reduce)
                    .partition(|e| matches!(e, Self::Const(_)))
            };
        }

        match self {
            Self::Var => Self::Var,
            Self::Const(c) => Self::Const(*c),
            Self::Add(exprs) => {
                let (consts, others): (Vec<_>, Vec<_>) = partition_consts!(exprs);

                let const_part: i64 = consts
                    .into_iter()
                    .map(|e| {
                        if let Self::Const(c) = e {
                            c
                        } else {
                            unreachable!()
                        }
                    })
                    .sum();

                if others.is_empty() {
                    return Self::Const(const_part);
                }

                let mut others = others;

                if const_part != 0 {
                    others.push(Self::Const(const_part));
                }

                if others.len() == 1 {
                    others.pop().unwrap()
                } else {
                    Self::Add(others)
                }
            }
            Self::Mul(exprs) => {
                let (consts, others): (Vec<_>, Vec<_>) = partition_consts!(exprs);

                let const_part: i64 = consts
                    .into_iter()
                    .map(|e| {
                        if let Self::Const(c) = e {
                            c
                        } else {
                            unreachable!()
                        }
                    })
                    .product();

                if others.is_empty() {
                    return Self::Const(const_part);
                }

                let mut others = others;

                if const_part != 1 {
                    others.push(Self::Const(const_part));
                }

                if others.len() == 1 {
                    others.pop().unwrap()
                } else {
                    Self::Mul(others)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        assert_eq!(
            Expression::Add(vec![]).reduce(),
            Expression::Const(0).reduce()
        );

        assert_eq!(
            Expression::Add(vec![Expression::Const(5), Expression::Const(7),]).reduce(),
            Expression::Const(12).reduce()
        );

        assert_eq!(
            Expression::Mul(vec![
                Expression::Add(vec![Expression::Const(5)]),
                Expression::Add(vec![]),
            ])
            .reduce(),
            Expression::Const(0).reduce(),
        );

        assert_eq!(
            Expression::Mul(vec![
                Expression::Const(5),
                Expression::Var,
                Expression::Add(vec![Expression::Const(2), Expression::Const(3)])
            ])
            .reduce(),
            Expression::Mul(vec![Expression::Const(25), Expression::Var,]).reduce(),
        );
    }
}
