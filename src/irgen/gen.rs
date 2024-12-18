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
        Ok(self.unary_exp.generate_koopa(program, env)?)
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
