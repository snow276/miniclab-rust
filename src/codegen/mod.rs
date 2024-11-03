mod gen;

use gen::GenerateAsm;
use koopa::ir::Program;

pub fn generate_riscv(program: &Program) -> String{
    let mut riscv_text = String::new();
    program.generate_riscv(&mut riscv_text);
    riscv_text
}
