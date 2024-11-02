mod gen;

use crate::ast::CompUnit;
use gen::GenerateKoopa;
use koopa::ir::Program;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_koopa_program(comp_unit: &CompUnit) -> Program {
    let mut program = Program::new();
    comp_unit.generate_koopa(&mut program);
    program
}
