use super::{env::IrgenEnv, IrgenError};
use crate::ast::*;
use koopa::ir::{builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder}, BinaryOp, FunctionData, Program, Type, Value};
use std::result::Result;

pub trait GenerateKoopa<'ast> {
    type Out;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError>;
}

impl<'ast> GenerateKoopa<'ast> for CompUnit {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        self.func_def.generate_koopa(program, env)?;
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        let params_ty = vec![];
        let ret_ty = self.func_type.generate_koopa(program, env)?;
        let func = program.new_func(FunctionData::new(
            format!("@{}", self.ident), 
            params_ty, 
            ret_ty
        ));
        let func_data = program.func_mut(func);

        let entry = func_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));

        func_data.layout_mut().bbs_mut().extend([entry]);

        env.set_cur_func(func);
        env.set_cur_bb(entry);

        self.block.stmt.generate_koopa(program, env)?;

        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncType {
    type Out = Type;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Int => Ok(Type::get_i32()),
            _ => Err(IrgenError::UnknownType),
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for Stmt {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        let ret_val = self.exp.generate_koopa(program, env)?;
        let cur_func = env.get_cur_func().unwrap();
        let cur_func_data = program.func_mut(*cur_func);
        let ret = cur_func_data.dfg_mut().new_value().ret(Some(ret_val));
        let cur_bb = env.get_cur_bb().unwrap();
        cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(ret).unwrap();
        
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for Exp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        Ok(self.l_or_exp.generate_koopa(program, env)?)
    }
}

impl<'ast> GenerateKoopa<'ast> for UnaryExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::PrimaryExp(primary_exp) => {
                Ok(primary_exp.generate_koopa(program, env)?)
            },
            Self::UnaryExp(op, unary_exp) => {
                let exp = unary_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let zero = cur_func_data.dfg_mut().new_value().integer(0);
                let value = match op {
                    UnaryOp::Plus => {
                        return Ok(exp);
                    },
                    UnaryOp::Minus => {
                        cur_func_data.dfg_mut().new_value().binary(BinaryOp::Sub, zero, exp)
                    },
                    UnaryOp::Not => {
                        cur_func_data.dfg_mut().new_value().binary(BinaryOp::Eq, exp, zero)
                    },
                };
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for PrimaryExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Exp(exp) => {
                Ok(exp.generate_koopa(program, env)?)
            },
            Self::Number(num) => {
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                Ok(cur_func_data.dfg_mut().new_value().integer(*num))
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for MulExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::UnaryExp(unary_exp) => {
                Ok(unary_exp.generate_koopa(program, env)?)
            },
            Self::Mul(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Mul, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Div(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Div, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Mod(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Mod, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for AddExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::MulExp(mul_exp) => {
                Ok(mul_exp.generate_koopa(program, env)?)
            },
            Self::Add(add_exp, mul_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = mul_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Add, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Sub(add_exp, mul_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = mul_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Sub, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for RelExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::AddExp(add_exp) => {
                Ok(add_exp.generate_koopa(program, env)?)
            },
            Self::Lt(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Lt, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Gt(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Gt, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Le(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Le, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
            Self::Ge(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Ge, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }  
    }
}

impl<'ast> GenerateKoopa<'ast> for EqExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::RelExp(rel_exp) => {
                Ok(rel_exp.generate_koopa(program, env)?)
            },
            Self::Eq(eq_exp, rel_exp) => {
                let lhs = eq_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Eq, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Ne(eq_exp, rel_exp) => {
                let lhs = eq_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::NotEq, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for LAndExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::EqExp(eq_exp) => {
                Ok(eq_exp.generate_koopa(program, env)?)
            },
            Self::And(l_and_exp, eq_exp) => {
                let lhs = l_and_exp.generate_koopa(program, env)?;
                let rhs = eq_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::And, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for LOrExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv) -> Result<Self::Out, IrgenError> {
        match self {
            Self::LAndExp(l_and_exp) => {
                Ok(l_and_exp.generate_koopa(program, env)?)
            }
            Self::Or(l_or_exp, l_and_exp) => {
                let lhs = l_or_exp.generate_koopa(program, env)?;
                let rhs = l_and_exp.generate_koopa(program, env)?;
                let cur_func = env.get_cur_func().unwrap();
                let cur_func_data = program.func_mut(*cur_func);
                let value = cur_func_data.dfg_mut().new_value().binary(BinaryOp::Or, lhs, rhs);
                let cur_bb = env.get_cur_bb().unwrap();
                cur_func_data.layout_mut().bb_mut(*cur_bb).insts_mut().push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}
