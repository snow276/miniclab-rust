use koopa::ir::Value;

use super::IrgenError;

pub enum ExpType {
    Int(Value),
    Void,
}

impl ExpType {
    pub fn to_int(self) -> Result<Value, IrgenError> {
        match self {
            Self::Int(val) => Ok(val),
            Self::Void => Err(IrgenError::UsingVoidValue),
        }
    }
}