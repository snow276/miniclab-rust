use super::CodegenError;
use koopa::ir::{FunctionData, Program, ValueKind};
use std::result::Result;

pub trait GenerateAsm {
    type Out;

    fn generate_riscv(&self, riscv_text: &mut String) -> Result<Self::Out, CodegenError>;
}

impl GenerateAsm for Program {
    type Out = ();
    
    fn generate_riscv(&self, riscv_text: &mut String) -> Result<Self::Out, CodegenError> {
        riscv_text.push_str("  .text\n");
        for &func in self.func_layout() {
            self.func(func).generate_riscv(riscv_text)?;
        }
        Ok(())
    }
}

impl GenerateAsm for FunctionData {
    type Out = ();

    fn generate_riscv(&self, riscv_text: &mut String) -> Result<Self::Out, CodegenError> {
        let func_name = &self.name()[1..];
        riscv_text.push_str(&format!("  .globl {}\n", func_name));
        riscv_text.push_str(&format!("{}:\n", func_name));

        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                match value_data.kind() {
                    ValueKind::Return(ret) => {
                        if let Some(value) = ret.value() {
                            let ret_val = self.dfg().value(value);
                            if let ValueKind::Integer(i) = ret_val.kind() {
                                riscv_text.push_str(&format!("  li a0, {}\n", i.value()));
                            }
                        }
                        else {
                            return Err(CodegenError::UnknownInstruction);
                        }
                        riscv_text.push_str("  ret\n");
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}

// impl GenerateAsm for ValueData {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String) -> Self::Out {
//         match self.kind() {
//             ValueKind::Integer(i) => {
//                 riscv_text.push_str(&format!("  li {}, {}\n", self.name(), i));
//             }
//             ValueKind::Return(ret) => {
//                 riscv_text.push_str(&format!("  ret {}\n", ret.name()));
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// impl GenerateAsm for values::Integer {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String) -> Self::Out {
//         riscv_text.push_str(&format!("  li {}, {}\n", self.name(), self.value()));
//     }
// }

// impl GenerateAsm for values::Return {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String) -> Self::Out {
//         if let Some(value) = self.value() {
//             riscv_text.push_str(&format!("  li a0, {}\n", value.()));
//         }
//         riscv_text.push_str(&format!("  li a0, {}\n", self.value()));
//         riscv_text.push_str("  ret\n");
//     }
// }
