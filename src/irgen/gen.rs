use super::{env::IrgenEnv, eval::Evaluate, symbol::SymbolInfo, IrgenError};
use crate::ast::*;
use koopa::ir::{builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder}, BinaryOp, FunctionData, Program, Type, Value};
use std::result::Result;

pub trait GenerateKoopa<'ast> {
    type Out;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError>;
}

impl<'ast> GenerateKoopa<'ast> for CompUnit {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        self.func_def.generate_koopa(program, env)?;
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for Decl {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::ConstDecl(const_decl) => {
                const_decl.generate_koopa(program, env)
            },
            Self::VarDecl(var_decl) => {
                var_decl.generate_koopa(program, env)
            },
        }        
    }
}

impl<'ast> GenerateKoopa<'ast> for ConstDecl {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        for const_def in &self.const_def_list {
            const_def.generate_koopa(program, env)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for BType {
    type Out = Type;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Int => Ok(Type::get_i32()),
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for ConstDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        let const_val = self.const_init_val.evaluate(env)?;
        if env.contains_symbol_in_cur_scope(&self.ident) {
            return Err(IrgenError::SymbolDeclaredMoreThanOnce);
        }
        env.new_symbol_const(&self.ident, const_val);
        Ok(())
    }   
}

impl<'ast> GenerateKoopa<'ast> for VarDecl {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        for var_def in &self.var_def_list {
            var_def.generate_koopa(program, env)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for VarDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        if env.contains_symbol_in_cur_scope(&self.ident) {
            return Err(IrgenError::SymbolDeclaredMoreThanOnce);
        }
        let ty = self.b_type.generate_koopa(program, env)?;
        let alloc = env.new_value(program).alloc(ty);
        env.dfg_mut(program).set_value_name(alloc, Some(format!("@{}_{}", self.ident, env.get_cur_scope_id())));
        env.new_inst(program).push_key_back(alloc).unwrap(); 
        env.new_symbol_var(&self.ident, alloc);   

        if let Some(init_val) = self.init_val.as_ref() {
            let val = init_val.generate_koopa(program, env)?;
            let store = env.new_value(program).store(val, alloc);
            env.new_inst(program).push_key_back(store).unwrap();
        }
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for InitVal {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        self.exp.generate_koopa(program, env)
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        let params_ty = vec![];
        let ret_ty = self.func_type.generate_koopa(program, env)?;
        let func = program.new_func(FunctionData::new(
            format!("@{}", self.ident), 
            params_ty, 
            ret_ty.clone()
        ));
        let func_data = program.func_mut(func);

        let entry = func_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));

        func_data.layout_mut().bbs_mut().extend([entry]);

        env.set_cur_func(func);
        env.set_cur_bb(entry);
        env.push_scope();

        let alloc_ret = env.new_value(program).alloc(ret_ty);
        env.new_inst(program).push_key_back(alloc_ret).unwrap();
        env.dfg_mut(program).set_value_name(alloc_ret, Some("%ret".into()));
        env.new_symbol_var("%ret", alloc_ret);

        self.block.generate_koopa(program, env)?;
        env.pop_scope();

        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncType {
    type Out = Type;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Int => Ok(Type::get_i32()),
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for Block {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        for block_item in &self.block_item_list {
            if env.is_cur_bb_returned() {
                break;
            }
            block_item.generate_koopa(program, env)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for BlockItem {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Stmt(stmt) => {
                stmt.generate_koopa(program, env)
            },
            Self::Decl(decl) => {
                decl.generate_koopa(program, env)
            },
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for Stmt {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Assign(l_val, exp) => {
                let val = exp.generate_koopa(program, env)?;
                if let Some(symbol_info) = env.get_symbol(&l_val.ident) {
                    match symbol_info {
                        SymbolInfo::Const(_) => {
                            return Err(IrgenError::AssignToConst);
                        },
                        SymbolInfo::Variable(alloc) => {
                            let store = env.new_value(program).store(val, *alloc);
                            env.new_inst(program).push_key_back(store).unwrap();
                        }
                    }
                } else {
                    return Err(IrgenError::SymbolUndeclared);
                }
            },
            Self::Exp(exp) => {
                if let Some(exp) = exp.as_ref() {
                    exp.generate_koopa(program, env)?;
                }
            }
            Self::Block(block) => {
                env.push_scope();
                block.generate_koopa(program, env)?;
                env.pop_scope();
            },
            Self::Return(exp) => {
                let ret_val = env.get_symbol("%ret").unwrap();
                let ret_val = match ret_val {
                    SymbolInfo::Variable(alloc) => *alloc,
                    _ => unreachable!()
                };
                match exp.as_ref() {
                    Some(exp) => {
                        let val = exp.generate_koopa(program, env)?;
                        let store = env.new_value(program).store(val, ret_val);
                        env.new_inst(program).push_key_back(store).unwrap();
                    },
                    None => {}
                }
                let load = env.new_value(program).load(ret_val);
                let ret = env.new_value(program).ret(Some(load));
                env.new_inst(program).push_key_back(load).unwrap();
                env.new_inst(program).push_key_back(ret).unwrap();
                env.set_cur_bb_returned(true);
            },
        }
        
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for Exp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        self.l_or_exp.generate_koopa(program, env)
    }
}

impl<'ast> GenerateKoopa<'ast> for LVal {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        if let Some(symbol_info) = env.get_symbol(&self.ident) {
            match symbol_info {
                SymbolInfo::Const(val) => {
                    Ok(env.new_value(program).integer(*val))
                },
                SymbolInfo::Variable(alloc) => {
                    let load = env.new_value(program).load(*alloc);
                    env.new_inst(program).push_key_back(load).unwrap();
                    Ok(load)
                }
            }
        } else {
            Err(IrgenError::SymbolUndeclared)
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for UnaryExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::PrimaryExp(primary_exp) => {
                primary_exp.generate_koopa(program, env)
            },
            Self::UnaryExp(op, unary_exp) => {
                let exp = unary_exp.generate_koopa(program, env)?;
                let zero = env.new_value(program).integer(0);
                let value = match op {
                    UnaryOp::Plus => {
                        return Ok(exp);
                    },
                    UnaryOp::Minus => {
                        env.new_value(program).binary(BinaryOp::Sub, zero, exp)
                    },
                    UnaryOp::Not => {
                        env.new_value(program).binary(BinaryOp::Eq, exp, zero)
                    },
                };
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for PrimaryExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Exp(exp) => {
                exp.generate_koopa(program, env)
            },
            Self::LVal(l_val) => {
                l_val.generate_koopa(program, env)
            },
            Self::Number(num) => {
                Ok(env.new_value(program).integer(*num))
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for MulExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::UnaryExp(unary_exp) => {
                unary_exp.generate_koopa(program, env)
            },
            Self::Mul(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Mul, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Div(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Div, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Mod(mul_exp, unary_exp) => {
                let lhs = mul_exp.generate_koopa(program, env)?;
                let rhs = unary_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Mod, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for AddExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::MulExp(mul_exp) => {
                mul_exp.generate_koopa(program, env)
            },
            Self::Add(add_exp, mul_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = mul_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Add, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Sub(add_exp, mul_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = mul_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Sub, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for RelExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::AddExp(add_exp) => {
                add_exp.generate_koopa(program, env)
            },
            Self::Lt(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Lt, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Gt(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Gt, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Le(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Le, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
            Self::Ge(add_exp, rel_exp) => {
                let lhs = add_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Ge, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }  
    }
}

impl<'ast> GenerateKoopa<'ast> for EqExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::RelExp(rel_exp) => {
                rel_exp.generate_koopa(program, env)
            },
            Self::Eq(eq_exp, rel_exp) => {
                let lhs = eq_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::Eq, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            },
            Self::Ne(eq_exp, rel_exp) => {
                let lhs = eq_exp.generate_koopa(program, env)?;
                let rhs = rel_exp.generate_koopa(program, env)?;
                let value = env.new_value(program).binary(BinaryOp::NotEq, lhs, rhs);
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for LAndExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::EqExp(eq_exp) => {
                eq_exp.generate_koopa(program, env)
            },
            Self::And(l_and_exp, eq_exp) => {
                let lhs = l_and_exp.generate_koopa(program, env)?;
                let rhs = eq_exp.generate_koopa(program, env)?;
                let zero = env.new_value(program).integer(0);
                let lhs_ne_zero = env.new_value(program).binary(BinaryOp::NotEq, lhs, zero);
                let rhs_ne_zero = env.new_value(program).binary(BinaryOp::NotEq, rhs, zero);
                let value = env.new_value(program).binary(BinaryOp::And, lhs_ne_zero, rhs_ne_zero);
                env.new_inst(program).push_key_back(lhs_ne_zero).unwrap();
                env.new_inst(program).push_key_back(rhs_ne_zero).unwrap();
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for LOrExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        match self {
            Self::LAndExp(l_and_exp) => {
                Ok(l_and_exp.generate_koopa(program, env)?)
            }
            Self::Or(l_or_exp, l_and_exp) => {
                let lhs = l_or_exp.generate_koopa(program, env)?;
                let rhs = l_and_exp.generate_koopa(program, env)?;
                let zero = env.new_value(program).integer(0);
                let lhs_ne_zero = env.new_value(program).binary(BinaryOp::NotEq, lhs, zero);
                let rhs_ne_zero = env.new_value(program).binary(BinaryOp::NotEq, rhs, zero);
                let value = env.new_value(program).binary(BinaryOp::Or, lhs_ne_zero, rhs_ne_zero);
                env.new_inst(program).push_key_back(lhs_ne_zero).unwrap();
                env.new_inst(program).push_key_back(rhs_ne_zero).unwrap();
                env.new_inst(program).push_key_back(value).unwrap();
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateKoopa<'ast> for ConstExp {
    type Out = Value;

    fn generate_koopa(&'ast self, program: &mut Program, env: &mut IrgenEnv<'ast>) -> Result<Self::Out, IrgenError> {
        self.exp.generate_koopa(program, env)
    }
}