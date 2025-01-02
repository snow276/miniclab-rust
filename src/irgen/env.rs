use koopa::ir::builder::{BlockBuilder, LocalBuilder};
use koopa::ir::dfg::DataFlowGraph;
use koopa::ir::entities::Function;
use koopa::ir::layout::{InstList, Layout};
use koopa::ir::{BasicBlock, Program, Type, Value};

use super::symbol::{SymbolInfo, SymbolTable};

pub struct IrgenEnv<'s> {
    cur_func: Option<Function>,
    cur_func_type: Option<Type>,
    cur_bb: Option<BasicBlock>,
    cur_bb_returned: bool,
    sym_tab: Vec<Box<SymbolTable<'s>>>,
    branch_id: i32,
    exit_bb: Option<BasicBlock>,
    and_id: i32,
    or_id: i32,
    while_id: i32,
    cur_while_cond_bb: Option<BasicBlock>,
    cur_while_end_bb: Option<BasicBlock>,
}

impl<'s> IrgenEnv<'s> {
    pub fn new() -> Self {
        Self { 
            cur_func: None, 
            cur_func_type: None,
            cur_bb: None, 
            cur_bb_returned: false, 
            sym_tab: Vec::new(), 
            branch_id: 0, 
            exit_bb: None,
            and_id: 0,
            or_id: 0,
            while_id: 0,
            cur_while_cond_bb: None,
            cur_while_end_bb: None,
        }
    }

    pub fn get_cur_func(&self) -> Option<&Function> {
        self.cur_func.as_ref()
    }

    pub fn set_cur_func(&mut self, func: Function) {
        self.cur_func = Some(func);
    }

    pub fn get_cur_func_type(&self) -> Option<&Type> {
        self.cur_func_type.as_ref()
    }

    pub fn set_cur_func_type(&mut self, ty: Type) {
        self.cur_func_type = Some(ty);
    }

    pub fn get_cur_bb(&self) -> Option<&BasicBlock> {
        self.cur_bb.as_ref()
    }

    pub fn set_cur_bb(&mut self, bb: BasicBlock) {
        self.cur_bb = Some(bb);
    }

    pub fn new_value(&self, program: &'s mut Program) -> LocalBuilder<'s> {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.dfg_mut().new_value()
    }

    pub fn new_bb(&self, program: &'s mut Program) -> BlockBuilder<'s> {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.dfg_mut().new_bb()
    }

    pub fn dfg_mut(&self, program: &'s mut Program) -> &'s mut DataFlowGraph {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.dfg_mut()
    }

    pub fn layout_mut(&self, program: &'s mut Program) -> &'s mut Layout {
        let cur_func = self.cur_func.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.layout_mut()
    }

    pub fn new_inst(&self, program: &'s mut Program) -> &'s mut InstList {
        let cur_func = self.cur_func.unwrap();
        let cur_bb = self.cur_bb.unwrap();
        let cur_func_data = program.func_mut(cur_func);
        cur_func_data.layout_mut().bb_mut(cur_bb).insts_mut()
    }

    pub fn push_scope(&mut self) {
        self.sym_tab.push(Box::new(SymbolTable::new()));
    }

    pub fn pop_scope(&mut self) {
        self.sym_tab.pop();
    }

    pub fn new_symbol_const(&mut self, ident: &'s str, val: i32) {
        let cur_sym_tab = self.sym_tab.last_mut().unwrap();
        cur_sym_tab.set_value(ident, SymbolInfo::Const(val));
    }

    pub fn new_symbol_var(&mut self, ident: &'s str, val: Value) {
        let cur_sym_tab = self.sym_tab.last_mut().unwrap();
        cur_sym_tab.set_value(ident, SymbolInfo::Variable(val));
    }

    pub fn contains_symbol_in_cur_scope(&self, ident: &'s str) -> bool {
        let cur_sym_tab = self.sym_tab.last().unwrap();
        cur_sym_tab.contains_key(ident)
    }

    pub fn get_symbol(&self, ident: &'s str) -> Option<&SymbolInfo> {
        for sym_tab in self.sym_tab.iter().rev() {
            if let Some(symbol_info) = sym_tab.get_value(ident) {
                return Some(symbol_info);
            }
        }
        None
    }

    pub fn new_func(&mut self, ident: &'s str, func: Function) {
        let global_sym_tab = self.sym_tab.first_mut().unwrap();
        global_sym_tab.set_value(ident, SymbolInfo::Function(func));
    }

    pub fn get_func(&self, ident: &'s str) -> Option<&Function> {
        let global_sym_tab = self.sym_tab.first().unwrap();
        if let Some(SymbolInfo::Function(func)) = global_sym_tab.get_value(ident) {
            Some(func)
        } else {
            None
        }
    }

    pub fn set_cur_bb_returned(&mut self, returned: bool) {
        self.cur_bb_returned = returned;
    }

    pub fn is_cur_bb_returned(&self) -> bool {
        self.cur_bb_returned
    }

    pub fn new_branch_id(&mut self) -> i32 {
        let branch_id = self.branch_id;
        self.branch_id += 1;
        branch_id
    }

    pub fn new_and_id(&mut self) -> i32 {
        let and_id = self.and_id;
        self.and_id += 1;
        and_id
    }

    pub fn new_or_id(&mut self) -> i32 {
        let or_id = self.or_id;
        self.or_id += 1;
        or_id
    }

    pub fn new_while_id(&mut self) -> i32 {
        let while_id = self.while_id;
        self.while_id += 1;
        while_id
    }

    pub fn set_exit_bb(&mut self, bb: BasicBlock) {
        self.exit_bb = Some(bb);
    }

    pub fn get_exit_bb(&self) -> Option<&BasicBlock> {
        self.exit_bb.as_ref()
    }

    pub fn set_cur_while_cond_bb(&mut self, bb: Option<BasicBlock>) {
        self.cur_while_cond_bb = bb;
    }

    pub fn get_cur_while_cond_bb(&self) -> Option<BasicBlock> {
        self.cur_while_cond_bb
    }

    pub fn set_cur_while_end_bb(&mut self, bb: Option<BasicBlock>) {
        self.cur_while_end_bb = bb;
    }

    pub fn get_cur_while_end_bb(&self) -> Option<BasicBlock> {
        self.cur_while_end_bb
    }
}
