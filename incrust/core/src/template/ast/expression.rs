use crate::args::Args;
use crate::evaluate::EvalResult;
use crate::value::Value;

pub enum Expression<'a> {
    Value {
        value: Value<'a>,
    },
    // UnOp {
    //     op: UnOp,
    //     right: Box<Expression>,
    // },
    BinOp {
        op: BinOp,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
}

pub enum UnOp {
    Neg,
    Not,
}

pub enum BinOp {
    // Arithmetic.
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    // // Boolean.
    // And,
    // Or,
    // // Comparison.
    // Eq,
    // Ne,
    // Gt,
    // Gte,
    // Lt,
    // Lte,
}

impl<'a> Expression<'a> {
    pub fn value(value: impl Into<Value<'a>>) -> Self {
        let value = value.into();
        Expression::Value { value }
    }

    pub fn bin_op(op: BinOp, left: Expression<'a>, right: Expression<'a>) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);
        Expression::BinOp { op, left, right }
    }

    pub fn eval(&'a self, args: &'a Args) -> EvalResult<Value<'a>> {
        Ok(match &self {
            Expression::Value { value } => value.copy_ref(),
            Expression::BinOp { op, left, right } => {
                let left = left.eval(args)?;
                let right = right.eval(args)?;
                match (left, right) {
                    (Value::Integer(left), Value::Integer(right)) => match op {
                        BinOp::Add => Value::Integer(left.add(&right)?),
                        BinOp::Sub => Value::Integer(left.sub(&right)?),
                        BinOp::Mul => Value::Integer(left.mul(&right)?),
                        BinOp::Div => Value::Integer(left.div(&right)?),
                        BinOp::Rem => Value::Integer(left.rem(&right)?),
                    },
                    _ => unimplemented!(),
                }
            }
        })
    }

    // pub fn evaluate(&self) -> EvalResult<Value> {
    //     Ok(match self {
    //         Expression::Value { value } => value.clone(),
    //         Expression::Sum { first, list } => {
    //             assert!(!list.is_empty());
    //             let mut sum = first.evaluate()?;
    //             for (op, x) in list {
    //                 let x = x.evaluate()?;
    //                 sum = match op {
    //                     SumOp::Add => sum.add(&x)?,
    //                     SumOp::Sub => sum.sub(&x)?,
    //                 };
    //             }
    //             sum
    //         }
    //         Expression::Prod { first, list } => {
    //             assert!(!list.is_empty());
    //             let mut prod = first.evaluate()?;
    //             for (op, x) in list {
    //                 let x = x.evaluate()?;
    //                 prod = match op {
    //                     ProdOp::Mul => prod.mul(&x)?,
    //                     ProdOp::Div => prod.div(&x)?,
    //                     ProdOp::Rem => prod.rem(&x)?,
    //                 };
    //             }
    //             prod
    //         }
    //     })
    // }
}
