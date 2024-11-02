use crate::ast::*;

use koopa::ir::{builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder}, FunctionData, Program, Type};

pub trait GenerateKoopa<'ast> {
    type Out;

    fn generate_koopa(&'ast self, program: &mut Program) -> Self::Out;
}

impl<'ast> GenerateKoopa<'ast> for CompUnit {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program) -> Self::Out {
        self.func_def.generate_koopa(program);
        ()
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncDef {
    type Out = ();

    fn generate_koopa(&'ast self, program: &mut Program) -> Self::Out {
        let params_ty = vec![];
        let ret_ty = self.func_type.generate_koopa(program);
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
        ()
    }
}

impl<'ast> GenerateKoopa<'ast> for FuncType {
    type Out = Type;

    fn generate_koopa(&'ast self, program: &mut Program) -> Self::Out {
        match self {
            Self::Int => Type::get_i32(),
        }
    }
}
