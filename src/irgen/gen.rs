use super::IrgenError;
use crate::ast::*;
use koopa::ir::{builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder}, FunctionData, Program, Type};
use std::result::Result;

pub trait GenerateKoopa<'ast> {
    type Out;

    fn generate_koopa(&'ast self, program: &mut Program) -> Result<Self::Out, IrgenError>;
}

impl<'ast> GenerateKoopa<'ast> for CompUnit {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program) -> Result<Self::Out, IrgenError> {
        self.func_def.generate_koopa(program)?;
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program) -> Result<Self::Out, IrgenError> {
        let params_ty = vec![];
        let ret_ty = self.func_type.generate_koopa(program)?;
        let func = program.new_func(FunctionData::new(
            format!("@{}", self.ident), 
            params_ty, 
            ret_ty
        ));
        let func_data = program.func_mut(func);

        let entry = func_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let ret_val = func_data.dfg_mut().new_value().integer(self.block.stmt.number);
        let ret = func_data.dfg_mut().new_value().ret(Some(ret_val));
        func_data.layout_mut().bbs_mut().extend([entry]);
        func_data.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret).unwrap();
        Ok(())
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncType {
    type Out = Type;

    fn generate_koopa(&'ast self, program: &mut Program) -> Result<Self::Out, IrgenError> {
        match self {
            Self::Int => Ok(Type::get_i32()),
            _ => Err(IrgenError::UnknownType),
        }
    }
}
