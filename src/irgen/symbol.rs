use std::collections::HashMap;

use koopa::ir::Value;

#[derive(Clone, Copy)]
pub enum SymbolInfo {
    Const(i32),
    Variable(Value), // This value should point to an "Alloc" in the IR.
}

pub struct SymbolTable<'s> {
    table: HashMap<&'s str, SymbolInfo>,
}

impl<'s> SymbolTable<'s> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, ident: &'s str, info: SymbolInfo) {
        self.table.insert(ident, info);
    }

    pub fn get_value(&self, ident: &'s str) -> Option<&SymbolInfo> {
        self.table.get(ident)
    }

    pub fn contains_key(&self, ident: &'s str) -> bool {
        self.table.contains_key(ident)
    }

}

