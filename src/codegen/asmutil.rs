use koopa::ir::{BasicBlock, BinaryOp, Value, ValueKind};

use super::env::CodegenEnv;


pub fn generate_li(riscv_text: &mut String, dest: &str, imm: i32) {
    riscv_text.push_str(&format!("  li {}, {}\n", dest, imm));
}

pub fn generate_add(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  add {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_addi(riscv_text: &mut String, dest: &str, src: &str, imm: i32) {
    assert!(imm >= -2048 && imm < 2048);
    riscv_text.push_str(&format!("  addi {}, {}, {}\n", dest, src, imm));
}

pub fn generate_sub(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  sub {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_mul(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  mul {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_div(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  div {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_mod(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  rem {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_and(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  and {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_or(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  or {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_eq(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  xor {}, {}, {}\n", dest, src1, src2));
    riscv_text.push_str(&format!("  seqz {}, {}\n", dest, dest));
}

pub fn generate_ne(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  xor {}, {}, {}\n", dest, src1, src2));
    riscv_text.push_str(&format!("  snez {}, {}\n", dest, dest));
}

pub fn generate_lt(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  slt {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_gt(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  sgt {}, {}, {}\n", dest, src1, src2));
}

pub fn generate_le(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  sgt {}, {}, {}\n", dest, src1, src2));
    riscv_text.push_str(&format!("  seqz {}, {}\n", dest, dest));
}

pub fn generate_ge(riscv_text: &mut String, dest: &str, src1: &str, src2: &str) {
    riscv_text.push_str(&format!("  slt {}, {}, {}\n", dest, src1, src2));
    riscv_text.push_str(&format!("  seqz {}, {}\n", dest, dest));
}

pub fn generate_lw(riscv_text: &mut String, dest: &str, base: &str, offset: i32) {
    assert!(offset >= -2048 && offset < 2048);
    riscv_text.push_str(&format!("  lw {}, {}({})\n", dest, offset, base));
}

pub fn generate_sw(riscv_text: &mut String, src: &str, base: &str, offset: i32) {
    assert!(offset >= -2048 && offset < 2048);
    riscv_text.push_str(&format!("  sw {}, {}({})\n", src, offset, base));
}

pub fn generate_bnez(riscv_text: &mut String, cond: &str, bb: &str) {
    riscv_text.push_str(&format!("  bnez {}, {}\n", cond, bb));
}

pub fn generate_j(riscv_text: &mut String, bb: &str) {
    riscv_text.push_str(&format!("  j {}\n", bb));
}

pub fn generate_addi_with_any_imm(riscv_text: &mut String, dest: &str, src: &str, tmp: &str, imm: i32) {
    if imm >= -2048 && imm < 2048 {
        generate_addi(riscv_text, dest, src, imm);
    } else {
        generate_li(riscv_text, tmp, imm);
        generate_add(riscv_text, dest, src, tmp);
    }
}

pub fn generate_lw_with_any_offset(riscv_text: &mut String, dest: &str, base: &str, tmp: &str, offset: i32) {
    if offset >= -2048 && offset < 2048 {
        generate_lw(riscv_text, dest, base, offset);
    } else {
        generate_li(riscv_text, tmp, offset);
        generate_add(riscv_text, tmp, base, tmp);
        generate_lw(riscv_text, dest, tmp, 0);
    }
}

pub fn generate_sw_with_any_offset(riscv_text: &mut String, src: &str, base: &str, tmp: &str, offset: i32) {
    if offset >= -2048 && offset < 2048 {
        generate_sw(riscv_text, src, base, offset);
    } else {
        generate_li(riscv_text, tmp, offset);
        generate_add(riscv_text, tmp, base, tmp);
        generate_sw(riscv_text, src, tmp, 0);
    }
}

pub fn generate_load(riscv_text: &mut String, env: &CodegenEnv, src: Value, dest: Value, tmp1: &str, tmp2: &str) {
    let offset = env.get_frame_size() - env.get_offset(src).unwrap();
    generate_lw_with_any_offset(riscv_text, tmp1, "sp", tmp2, offset);
    let offset = env.get_frame_size() - env.get_offset(dest).unwrap();
    generate_sw_with_any_offset(riscv_text, tmp1, "sp", tmp2, offset);
}

pub fn generate_store(riscv_text: &mut String, env: &CodegenEnv, src: Value, dest: Value, tmp1: &str, tmp2: &str) {
    let src_data = env.get_value_data(src);
    match src_data.kind() {
        ValueKind::Integer(i) => {
            generate_li(riscv_text, tmp1, i.value());
        }
        _ => {
            let offset = env.get_frame_size() - env.get_offset(src).unwrap();
            generate_lw_with_any_offset(riscv_text, tmp1, "sp", tmp2, offset);
        }
    }
    let offset = env.get_frame_size() - env.get_offset(dest).unwrap();
    generate_sw_with_any_offset(riscv_text, tmp1, "sp", tmp2, offset);
}

pub fn generate_binary(riscv_text: &mut String, env: &CodegenEnv, op: BinaryOp, lhs: Value, rhs: Value, dest: Value, tmp1: &str, tmp2: &str, tmp3: &str, tmp4: &str) {
    let lhs_data = env.get_value_data(lhs);
    match lhs_data.kind() {
        ValueKind::Integer(i) => {
            generate_li(riscv_text, tmp1, i.value());
        }
        _ => {
            let offset = env.get_frame_size() - env.get_offset(lhs).unwrap();
            generate_lw_with_any_offset(riscv_text, tmp1, "sp", tmp3, offset);
        }
    }
    let rhs_data = env.get_value_data(rhs);
    match rhs_data.kind() {
        ValueKind::Integer(i) => {
            generate_li(riscv_text, tmp2, i.value());
        }
        _ => {
            let offset = env.get_frame_size() - env.get_offset(rhs).unwrap();
            generate_lw_with_any_offset(riscv_text, tmp2, "sp", tmp4, offset);
        }
    }
    match op {
        BinaryOp::Add => generate_add(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Sub => generate_sub(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Mul => generate_mul(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Div => generate_div(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Mod => generate_mod(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::And => generate_and(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Or => generate_or(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Eq => generate_eq(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::NotEq => generate_ne(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Lt => generate_lt(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Gt => generate_gt(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Le => generate_le(riscv_text, tmp1, tmp1, tmp2),
        BinaryOp::Ge => generate_ge(riscv_text, tmp1, tmp1, tmp2),
        _ => unimplemented!()
    }
    let offset = env.get_frame_size() - env.get_offset(dest).unwrap();
    generate_sw_with_any_offset(riscv_text, tmp1, "sp", tmp3, offset);
}

pub fn generate_branch(riscv_text: &mut String, env: &CodegenEnv, cond: Value, bb_true: BasicBlock, bb_false: BasicBlock, tmp1: &str, tmp2: &str) {
    let cond_data = env.get_value_data(cond);
    let label_true = &env.get_label(bb_true)[1..];
    let label_false = &env.get_label(bb_false)[1..];
    match cond_data.kind() {
        ValueKind::Integer(i) => {
            if i.value() != 0 {
                riscv_text.push_str(&format!("  j {}\n", label_true));
            } else {
                riscv_text.push_str(&format!("  j {}\n", label_false));
            }
        }
        _ => {
            let offset = env.get_frame_size() - env.get_offset(cond).unwrap();
            generate_lw_with_any_offset(riscv_text, tmp1, "sp", tmp2, offset);
            generate_bnez(riscv_text, tmp1, label_true);
            generate_j(riscv_text, label_false); 
        }
    }
}

pub fn generate_jump(riscv_text: &mut String, env: &CodegenEnv, bb: BasicBlock) {
    let label = &env.get_label(bb)[1..];
    generate_j(riscv_text, label);
}

pub fn generate_return(riscv_text: &mut String, env: &CodegenEnv, ret: Value, dest: &str, tmp: &str) {
    let ret_data = env.get_value_data(ret);
    match ret_data.kind() {
        ValueKind::Integer(i) => {
            generate_li(riscv_text, "a0", i.value());
        }
        _ => {
            let offset = env.get_frame_size() - env.get_offset(ret).unwrap();
            generate_lw_with_any_offset(riscv_text, dest, "sp", tmp, offset);
        }
    }    
    generate_addi_with_any_imm(riscv_text, "sp", "sp", tmp, env.get_frame_size());
    riscv_text.push_str("  ret\n");
}