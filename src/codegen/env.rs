use koopa::ir::entities::Function;
use koopa::ir::Program;

pub struct CodegenEnv<'p> {
    program: &'p Program,
    cur_func: Option<Function>,
}

impl<'p, 'f> CodegenEnv<'p> {
    pub fn new(program: &'p Program) -> Self {
        Self { program, cur_func: None }
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
}
