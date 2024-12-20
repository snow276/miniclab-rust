mod env;
mod eval;
mod gen;
mod symbol;

use crate::ast::CompUnit;
use env::IrgenEnv;
use gen::GenerateKoopa;
use koopa::ir::Program;
use std::fmt;
use std::result::Result;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_koopa_program(comp_unit: &CompUnit) -> Result<Program, IrgenError> {
    let mut program = Program::new();
    comp_unit.generate_koopa(&mut program, &mut IrgenEnv::new())?;
    Ok(program)
}

pub enum IrgenError {
    UnknownType,
    SymbolDeclaredMoreThanOnce,
    SymbolUndeclared,
    AssignToConst,
    InitializeConstWithVariable,
}

impl fmt::Display for IrgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownType => write!(f, "Unknown variable type"),
            Self::SymbolDeclaredMoreThanOnce => write!(f, "Symbol declared more than once"),
            Self::SymbolUndeclared => write!(f, "Symbol undeclared"),
            Self::AssignToConst => write!(f, "Assigning to a const symbol"),
            Self::InitializeConstWithVariable => write!(f, "Initializing a const symbol with a variable"),
        }
    }
}

impl fmt::Debug for IrgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
