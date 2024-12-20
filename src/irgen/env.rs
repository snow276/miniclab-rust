use koopa::ir::entities::Function;
use koopa::ir::{BasicBlock, Value};

use super::symbol::{SymbolInfo, SymbolTable};

pub struct IrgenEnv<'s> {
    cur_func: Option<Function>,
    cur_bb: Option<BasicBlock>,
    cur_bb_returned: bool,
    sym_tab: SymbolTable<'s>,
}

impl<'s> IrgenEnv<'s> {
    pub fn new() -> Self {
        Self { cur_func: None, cur_bb: None, cur_bb_returned: false, sym_tab: SymbolTable::new() }
    }

    pub fn get_cur_func(&self) -> Option<&Function> {
        self.cur_func.as_ref()
    }

    pub fn set_cur_func(&mut self, func: Function) {
        self.cur_func = Some(func);
    }

    pub fn get_cur_bb(&self) -> Option<&BasicBlock> {
        self.cur_bb.as_ref()
    }

    pub fn set_cur_bb(&mut self, bb: BasicBlock) {
        self.cur_bb = Some(bb);
    }

    pub fn new_symbol_const(&mut self, ident: &'s str, val: i32) {
        self.sym_tab.set_value(ident, SymbolInfo::Const(val));
    }

    pub fn new_symbol_var(&mut self, ident: &'s str, val: Value) {
        self.sym_tab.set_value(ident, SymbolInfo::Variable(val));
    }

    pub fn contains_symbol(&self, ident: &'s str) -> bool {
        self.sym_tab.contains_key(ident)
    }

    pub fn get_symbol(&self, ident: &'s str) -> Option<&SymbolInfo> {
        self.sym_tab.get_value(ident)
    }

    pub fn set_cur_bb_returned(&mut self, returned: bool) {
        self.cur_bb_returned = returned;
    }

    pub fn is_cur_bb_returned(&self) -> bool {
        self.cur_bb_returned
    }
}
