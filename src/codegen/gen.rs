use super::asmutil::*;
use super::CodegenError;
use super::env::CodegenEnv;
use koopa::ir::entities::ValueData;
use koopa::ir::Value;
use koopa::ir::TypeKind;
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

        let mut frame_offset= 0;
        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                match value_data.ty().kind() {
                    TypeKind::Int32 | TypeKind::Pointer(_) => {
                        frame_offset += 4;
                        env.set_offset(inst, frame_offset);
                    },
                    TypeKind::Unit => {},
                    _ => unimplemented!()
                }
            }
        }
        frame_offset = (frame_offset + 15) & !15;
        env.set_frame_size(frame_offset);
        generate_addi_with_any_imm(riscv_text, "sp", "sp", "t0", -frame_offset);

        for (&bb, node) in self.layout().bbs() {
            let label = &env.get_label(bb)[1..];
            riscv_text.push_str(&format!("{}:\n", label));
            for &inst in node.insts().keys() {
                inst.generate_riscv(riscv_text, env)?;
                // let value_data = self.dfg().value(inst);
                // value_data.generate_riscv(riscv_text, env)?;
            }
        }
        Ok(())
    }
}

impl<'p> GenerateAsm<'p> for Value {
    type Out = ();

    fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
        let value_data = env.get_value_data(*self);
        match value_data.kind() {
            ValueKind::Alloc(_) => {}
            ValueKind::Binary(binary) => {
                generate_binary(riscv_text, env, binary.op(), binary.lhs(), binary.rhs(), *self, "t0", "t1", "t2", "t3");                
            }
            ValueKind::Branch(branch) => {
                generate_branch(riscv_text, env, branch.cond(), branch.true_bb(), branch.false_bb(), "t0", "t1");
            }
            ValueKind::Jump(jump) => {
                generate_jump(riscv_text, env, jump.target());
            }
            ValueKind::Load(load) => {
                generate_load(riscv_text, env, load.src(), *self, "t0", "t1");
            }
            ValueKind::Store(store) => {
                generate_store(riscv_text, env, store.value(), store.dest(), "t0", "t1");
            }
            ValueKind::Return(ret) => {
                if let Some(ret_val) = ret.value() {
                    generate_return(riscv_text, env, ret_val, "a0", "t0");
                } else {
                    return Err(CodegenError::MissingReturnValue);
                }
            }
            _ => unimplemented!()
        }
        Ok(())
    }
}

// impl<'p> GenerateAsm<'p> for ValueData {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
//         match self.kind() {
//             // ValueKind::Integer(i) => {
//             //     riscv_text.push_str(&format!("  li {}, {}\n", self.name(), i));
//             // }
//             ValueKind::Store(store) => {
//                 store.generate_riscv(riscv_text, env)?;
//             }
//             ValueKind::Load(load) => {
//                 load.generate_riscv(riscv_text, env)?;
//             }
//             ValueKind::Return(ret) => {
//                 ret.generate_riscv(riscv_text, env)?;
//             }
//             _ => unreachable!(),
//         }
//         Ok(())
//     }
// }

// impl GenerateAsm for Integer {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String) -> Self::Out {
//         riscv_text.push_str(&format!("  li {}, {}\n", self.name(), self.value()));
//     }
// }

// impl<'p> GenerateAsm<'p> for Store {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
//         let value = self.value();
//         let cur_func = env.get_cur_func().unwrap();
//         let cur_func_data = env.get_program().func(*cur_func);
//         let value_data = cur_func_data.dfg().value(value);
//         match value_data.kind() {
//             ValueKind::Integer(i) => {
//                 generate_li(riscv_text, "t0", i.value());
//             }
//             _ => {
//                 let offset = env.get_frame_size() - env.get_offset(value).unwrap();
//                 generate_lw_with_any_offset(riscv_text, "t0", "sp", "t1", offset);
//             }
//         }
//         let dest = self.dest();
//         let offset = env.get_frame_size() - env.get_offset(dest).unwrap();
//         generate_sw_with_any_offset(riscv_text, "t0", "sp", "t1", offset);
//         Ok(())
//     }
// }

// impl<'p> GenerateAsm<'p> for Return {
//     type Out = ();

//     fn generate_riscv(&self, riscv_text: &mut String, env: &mut CodegenEnv<'p>) -> Result<Self::Out, CodegenError> {
//         if let Some(value) = self.value() {
//             let cur_func = env.get_cur_func().unwrap();
//             let cur_func_data = env.get_program().func(*cur_func);
//             let ret_val = cur_func_data.dfg().value(value);
//             if let ValueKind::Integer(i) = ret_val.kind() {
//                 riscv_text.push_str(&format!("  li a0, {}\n", i.value()));
//             }
//             else {
//                 return Err(CodegenError::UnknownInstruction);
//             }
//         }
//         riscv_text.push_str("  ret\n");
//         Ok(())
//     }
// }
