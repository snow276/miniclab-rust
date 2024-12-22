mod asmutil;
mod env;
mod gen;

use env::CodegenEnv;
use gen::GenerateAsm;
use koopa::ir::Program;
use std::fmt;
use std::result::Result;

/// Generates the given Koopa IR program to RISC-V assembly.
pub fn generate_riscv(program: &Program) -> Result<String, CodegenError> {
    let mut riscv_text = String::new();
    program.generate_riscv(&mut riscv_text, &mut CodegenEnv::new(program))?;
    Ok(riscv_text)
}

pub enum CodegenError {
    UnknownInstruction,
    MissingReturnValue,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::UnknownInstruction => write!(f, "Unknown instruction"),
        Self::MissingReturnValue => write!(f, "Missing return value"),
      }
    }
  }

impl fmt::Debug for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

