mod gen;

use crate::ast::CompUnit;
use gen::GenerateKoopa;
use koopa::ir::Program;
use std::fmt;
use std::result::Result;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_koopa_program(comp_unit: &CompUnit) -> Result<Program, IrgenError> {
    let mut program = Program::new();
    comp_unit.generate_koopa(&mut program)?;
    Ok(program)
}

pub enum IrgenError {
    UnknownType,
}

impl fmt::Display for IrgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::UnknownType => write!(f, "Unknown variable type"),
      }
    }
  }

impl fmt::Debug for IrgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
