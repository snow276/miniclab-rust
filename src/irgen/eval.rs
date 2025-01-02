use crate::ast::*;

use super::{env::IrgenEnv, symbol::SymbolInfo, IrgenError};


pub trait Evaluate {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError>;
}

impl Evaluate for LVal {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        if let Some(symbol_info) = env.get_symbol(&self.ident) {
            match symbol_info {
                SymbolInfo::Const(val) => Ok(*val),
                SymbolInfo::Variable(_) => Err(IrgenError::InitializeConstWithVariable),
                SymbolInfo::Function(_) => Err(IrgenError::InitializeConstWithFunctionCall),
            }
        } else {
            Err(IrgenError::SymbolUndeclared)
        }
    }
}

impl Evaluate for ConstInitVal {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        self.const_exp.evaluate(env)
    }
}

impl Evaluate for ConstExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        self.exp.evaluate(env)
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        self.l_or_exp.evaluate(env)
    }
}

impl Evaluate for LOrExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::LAndExp(l_and_exp) => l_and_exp.evaluate(env),
            Self::Or(l_or_exp, l_and_exp) => {
                let lhs = l_or_exp.evaluate(env)?;
                let rhs = l_and_exp.evaluate(env)?;
                Ok((lhs != 0 || rhs != 0) as i32)
            }
        }       
    }
}

impl Evaluate for LAndExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::EqExp(eq_exp) => eq_exp.evaluate(env),
            Self::And(l_and_exp, eq_exp) => {
                let lhs = l_and_exp.evaluate(env)?;
                let rhs = eq_exp.evaluate(env)?;
                Ok((lhs != 0 && rhs != 0) as i32)
            }
        }
    }
}

impl Evaluate for EqExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::RelExp(rel_exp) => rel_exp.evaluate(env),
            Self::Eq(eq_exp, rel_exp) => {
                let lhs = eq_exp.evaluate(env)?;
                let rhs = rel_exp.evaluate(env)?;
                Ok((lhs == rhs) as i32)
            }
            Self::Ne(eq_exp, rel_exp) => {
                let lhs = eq_exp.evaluate(env)?;
                let rhs = rel_exp.evaluate(env)?;
                Ok((lhs != rhs) as i32)
            }
        }
    }
}

impl Evaluate for RelExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::AddExp(add_exp) => add_exp.evaluate(env),
            Self::Lt(rel_exp, add_exp) => {
                let lhs = rel_exp.evaluate(env)?;
                let rhs = add_exp.evaluate(env)?;
                Ok((lhs < rhs) as i32)
            }
            Self::Gt(rel_exp, add_exp) => {
                let lhs = rel_exp.evaluate(env)?;
                let rhs = add_exp.evaluate(env)?;
                Ok((lhs > rhs) as i32)
            }
            Self::Le(rel_exp, add_exp) => {
                let lhs = rel_exp.evaluate(env)?;
                let rhs = add_exp.evaluate(env)?;
                Ok((lhs <= rhs) as i32)
            }
            Self::Ge(rel_exp, add_exp) => {
                let lhs = rel_exp.evaluate(env)?;
                let rhs = add_exp.evaluate(env)?;
                Ok((lhs >= rhs) as i32)
            }
        }
    }
    
}

impl Evaluate for AddExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::MulExp(mul_exp) => mul_exp.evaluate(env),
            Self::Add(add_exp, mul_exp) => {
                let lhs = add_exp.evaluate(env)?;
                let rhs = mul_exp.evaluate(env)?;
                Ok(lhs + rhs)
            }
            Self::Sub(add_exp, mul_exp) => {
                let lhs = add_exp.evaluate(env)?;
                let rhs = mul_exp.evaluate(env)?;
                Ok(lhs - rhs)
            }
        }
    }
}

impl Evaluate for MulExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::UnaryExp(unary_exp) => unary_exp.evaluate(env),
            Self::Mul(mul_exp, unary_exp) => {
                let lhs = mul_exp.evaluate(env)?;
                let rhs = unary_exp.evaluate(env)?;
                Ok(lhs * rhs)
            }
            Self::Div(mul_exp, unary_exp) => {
                let lhs = mul_exp.evaluate(env)?;
                let rhs = unary_exp.evaluate(env)?;
                Ok(lhs / rhs)
            }
            Self::Mod(mul_exp, unary_exp) => {
                let lhs = mul_exp.evaluate(env)?;
                let rhs = unary_exp.evaluate(env)?;
                Ok(lhs % rhs)
            }
        }
    }
}

impl Evaluate for UnaryExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::PrimaryExp(primary_exp) => primary_exp.evaluate(env),
            Self::FuncCall(_, _) => Err(IrgenError::UseFunctionAsVariable),
            Self::UnaryExp(op, unary_exp) => {
                let val = unary_exp.evaluate(env)?;
                match op {
                    UnaryOp::Plus => Ok(val),
                    UnaryOp::Minus => Ok(-val),
                    UnaryOp::Not => Ok((val == 0) as i32)
                }
            }
        }
    }
}

impl Evaluate for PrimaryExp {
    fn evaluate(&self, env: &IrgenEnv) -> Result<i32, IrgenError> {
        match self {
            Self::Exp(exp) => exp.evaluate(env),
            Self::LVal(lval) => lval.evaluate(env),
            Self::Number(num) => Ok(*num)
        }
    }
}

