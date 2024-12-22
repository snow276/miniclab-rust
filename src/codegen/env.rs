use std::collections::HashMap;

use koopa::ir::entities::{Function, ValueData};
use koopa::ir::{Program, Value};

pub struct CodegenEnv<'p> {
    program: &'p Program,
    cur_func: Option<Function>,
    stack_info: StackInfo,
}

impl<'p, 'f> CodegenEnv<'p> {
    pub fn new(program: &'p Program) -> Self {
        Self { program, cur_func: None , stack_info: StackInfo::new() }
    }

    pub fn get_program(&self) -> &'p Program {
        self.program
    }

    pub fn get_cur_func(&self) -> Option<&Function> {
        self.cur_func.as_ref()
    }

    pub fn set_cur_func(&mut self, func: Function) {
        self.cur_func = Some(func);
    }

    pub fn set_frame_size(&mut self, frame_size: i32) {
        self.stack_info.set_frame_size(frame_size);
    }

    pub fn get_frame_size(&self) -> i32 {
        self.stack_info.get_frame_size()
    }

    pub fn get_offset(&self, value: Value) -> Option<i32> {
        self.stack_info.get_offset(value)
    }

    pub fn set_offset(&mut self, value: Value, offset: i32) {
        self.stack_info.set_offset(value, offset);
    }

    pub fn get_value_data(&self, value: Value) -> &ValueData {
        let cur_func = self.cur_func.expect("No current function");
        let cur_func_data = self.program.func(cur_func);
        cur_func_data.dfg().value(value)
    }
}

pub struct StackInfo {
    frame_size: i32,
    offset_table: HashMap<Value, i32>,
}

impl StackInfo {
    pub fn new() -> Self {
        Self { frame_size: 0, offset_table: HashMap::new() }
    }

    pub fn set_frame_size(&mut self, frame_size: i32) {
        self.frame_size = frame_size;
    }

    pub fn get_frame_size(&self) -> i32 {
        self.frame_size
    }

    pub fn get_offset(&self, value: Value) -> Option<i32> {
        self.offset_table.get(&value).copied()
    }

    pub fn set_offset(&mut self, value: Value, offset: i32) {
        self.offset_table.insert(value, offset);
    }
}