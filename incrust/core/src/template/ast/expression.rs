use crate::evaluate::{EvalError, EvalResult};
use crate::value::Value;
use crate::{Context, Identifier};

pub enum Expression<'a> {
    Value {
        value: Value<'a>,
    },
    Var {
        name: Identifier,
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
    // Boolean.
    And,
    Or,
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

    pub fn var(name: impl Into<Identifier>) -> Self {
        let name = name.into();
        Expression::Var { name }
    }

    pub fn bin_op(op: BinOp, left: Expression<'a>, right: Expression<'a>) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);
        Expression::BinOp { op, left, right }
    }

    pub fn eval(&'a self, context: &'a Context<'a>) -> EvalResult<Value<'a>> {
        Ok(match &self {
            Expression::Value { value } => value.copy_ref(),
            Expression::Var { name } => context
                .var(name)
                .ok_or(EvalError::UnknownVariable)?
                .copy_ref(),
            Expression::BinOp { op, left, right } => {
                let left = left.eval(context)?;
                let right = right.eval(context)?;
                match (left, right) {
                    (Value::Boolean(left), Value::Boolean(right)) => match op {
                        BinOp::And => Value::Boolean(left && right),
                        BinOp::Or => Value::Boolean(left || right),
                        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                            Err(EvalError::NotAllowedOperation)?
                        }
                    },
                    (Value::Integer(left), Value::Integer(right)) => match op {
                        BinOp::Add => Value::Integer(left.add(&right)?),
                        BinOp::Sub => Value::Integer(left.sub(&right)?),
                        BinOp::Mul => Value::Integer(left.mul(&right)?),
                        BinOp::Div => Value::Integer(left.div(&right)?),
                        BinOp::Rem => Value::Integer(left.rem(&right)?),
                        BinOp::And | BinOp::Or => Err(EvalError::NotAllowedOperation)?,
                    },
                    _ => unimplemented!(),
                }
            }
        })
    }
}
