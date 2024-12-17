use super::CodegenError;
use super::env::CodegenEnv;
use koopa::ir::entities::ValueData;
use koopa::ir::values::*;
use koopa::ir::{FunctionData, Program, ValueKind};
use std::result::Result;

pub trait GenerateAsm<'p> {
    type Out;

    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError>;
}

impl<'p> GenerateAsm<'p> for Program {
    type Out = ();
    
    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
        riscv_text.push_str("  .text\n");
        for &func in self.func_layout() {
            env.set_cur_func(func);
            self.func(func).generate_riscv(riscv_text, env)?;
        }
        Ok(())
    }
}

impl<'p> GenerateAsm<'p> for FunctionData {
    type Out = ();

    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
        let func_name = &self.name()[1..];
        riscv_text.push_str(&format!("  .globl {}\n", func_name));
        riscv_text.push_str(&format!("{}:\n", func_name));

        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                value_data.generate_riscv(riscv_text, env)?;
            }
        }
        Ok(())
    }
}

impl<'p> GenerateAsm<'p> for ValueData {
    type Out = ();

    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
        match self.kind() {
            // ValueKind::Integer(i) => {
            //     riscv_text.push_str(&format!("  li {}, {}\n", self.name(), i));
            // }
            ValueKind::Return(ret) => {
                ret.generate_riscv(riscv_text, env)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

// impl GenerateAsm for Integer {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String) -> Self::Out {
//         riscv_text.push_str(&format!("  li {}, {}\n", self.name(), self.value()));
//     }
// }

impl<'p> GenerateAsm<'p> for Return {
    type Out = ();

    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
        if let Some(value) = self.value() {
            let cur_func = env.get_cur_func().unwrap();
            let cur_func_data = env.get_program().func(*cur_func);
            let ret_val = cur_func_data.dfg().value(value);
            if let ValueKind::Integer(i) = ret_val.kind() {
                riscv_text.push_str(&format!("  li a0, {}\n", i.value()));
            }
            else {
                return Err(CodegenError::UnknownInstruction);
            }
        }
        riscv_text.push_str("  ret\n");
        Ok(())
    }
}
